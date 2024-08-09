use std::fmt::Display;

#[cfg(feature = "rand")]
use crate::random_usize_vec;

use serde::Deserialize;

/// List of [Post]
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Posts(Vec<Post>);

/// # Main data structure
///
/// * Post data from reqwest, which contains urls as well
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Post {
    /// picture in miniature
    pub preview_url: String,
    /// picture resized by default
    /// W - `sample_wight`
    /// H - `sample_hight`
    pub sample_url: String,
    /// raw picture
    /// W - `wight`
    /// H - `hight`
    pub file_url: String,
    /// Hash of picture
    pub hash: String,
    /// W of raw picture (`file_url`)
    pub width: i64,
    /// H of raw picture (`file_url`)
    pub height: i64,
    /// Unique ID
    pub id: i64,
    /// File name
    pub image: String,
    /// Author
    pub owner: String,
    /// Linked ID
    pub parent_id: i64,
    /// has resized ?
    pub sample: bool,
    /// H of `sample_picture`
    pub sample_height: i64,
    /// W of `sample_picture`
    pub sample_width: i64,
    /// Home many people rate this image?
    pub score: i64,
    /// Tags of picture
    pub tags: String,
    /// Used for translation (in my case)
    pub has_notes: bool,
    // pub comment_count: i64,
}

/// Less info struct of [Post]
#[derive(Debug, Clone)]
pub struct MiniPost<'a> {
    pub id: u64,
    /// picture in miniature
    pub preview_url: &'a str,
    /// picture resized by default
    /// W - `sample_wight`
    /// H - `sample_hight`
    pub sample_url: &'a str,
    /// raw picture
    /// W - `wight`
    /// H - `hight`
    pub file_url: &'a str,
    /// W of raw picture (`file_url`)
    pub width: u64,
    /// H of raw picture (`file_url`)
    pub height: u64,
    /// H of `sample_picture`
    pub sample_height: u64,
    /// W of `sample_picture`
    pub sample_width: u64,
    pub tags: Vec<&'a str>,
}

impl<'a> From<&'a Post> for MiniPost<'a> {
    fn from(val: &'a Post) -> Self {
        MiniPost {
            id: val.id as u64,
            preview_url: &val.preview_url,
            sample_url: &val.sample_url,
            file_url: &val.file_url,
            width: val.width as u64,
            height: val.height as u64,
            sample_height: val.sample_height as u64,
            sample_width: val.sample_width as u64,
            tags: val.tags.split(' ').collect(),
        }
    }
}

impl<'a> Display for MiniPost<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, tags: {:#?}, url: {}",
            self.id, self.tags, self.file_url
        )
    }
}

impl<'a> From<&'a Posts> for MiniPosts<'a> {
    fn from(value: &'a Posts) -> Self {
        let a: Vec<MiniPost> = value.0.iter().map(|x| x.into()).collect();
        Self(a)
    }
}

#[derive(Debug, Default)]
pub struct MiniPosts<'a>(Vec<MiniPost<'a>>);

// Реализация From для автоматической конвертации
impl<'a> From<Vec<MiniPost<'a>>> for MiniPosts<'a> {
    #[inline]
    fn from(value: Vec<MiniPost<'a>>) -> Self {
        Self(value)
    }
}

impl From<Vec<Post>> for Posts {
    #[inline]
    fn from(value: Vec<Post>) -> Self {
        Self(value)
    }
}

// Реализация From для Vec::new()
impl<'a> From<()> for MiniPosts<'a> {
    #[inline]
    fn from(_: ()) -> Self {
        MiniPosts::default()
    }
}

// Реализация From для Vec::new()
impl From<()> for Posts {
    #[inline]
    fn from(_: ()) -> Self {
        Posts::default()
    }
}

impl<'a> MiniPosts<'a> {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// get `preview_url` of all posts
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_p_urls();
    /// }
    /// ```
    pub fn get_p_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(x.preview_url));
        urls
    }
    /// get `sample_url` of all posts
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_s_urls();
    /// }
    /// ```
    pub fn get_s_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(x.sample_url));
        urls
    }
    /// get `file_url` of all posts
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_f_urls();
    /// }
    /// ```
    pub fn get_f_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(x.file_url));
        urls
    }

    /// get first `preview_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_p_url();
    /// }
    /// ```
    #[inline]
    pub fn get_p_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.preview_url)
    }
    /// get first `sample_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_s_url();
    /// }
    /// ```
    #[inline]
    pub fn get_s_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.sample_url)
    }
    /// get first `file_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_f_url();
    /// }
    /// ```
    #[inline]
    pub fn get_f_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.file_url)
    }

    /// Make ref from [MiniPost]
    #[inline]
    pub fn data(&self) -> &Vec<MiniPost> {
        &self.0
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique file_urls (raw urls) as string slices (`&str`).
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_p_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_f_urls(&self, size: usize) -> Vec<&str> {
        random_usize_vec!(self.get_f_urls(), size)
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique preview_urls as string slices (`&str`).
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_p_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_p_urls(&self, size: usize) -> Vec<&str> {
        random_usize_vec!(self.get_p_urls(), size)
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique sample urls as string slices (`&str`).
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_s_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_s_urls(&self, size: usize) -> Vec<&str> {
        random_usize_vec!(self.get_s_urls(), size)
    }

    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random(&self, size: usize) -> MiniPosts<'a> {
        use crate::random_usize_vec_cloned;
        random_usize_vec_cloned!(self.0, size)
    }

    #[cfg(feature = "rand")]
    #[inline]
    pub fn shuffle(&self) -> MiniPosts<'a> {
        use crate::random_usize_vec_cloned;
        random_usize_vec_cloned!(self.0, self.0.len())
    }

    #[inline]
    pub fn make_detail_list(&self) -> Vec<String> {
        self.0.iter().map(|x| x.to_string()).collect()
    }
}

impl<'a> From<MiniPosts<'a>> for Vec<String> {
    fn from(val: MiniPosts<'a>) -> Self {
        val.0.iter().map(|x| format!("{}", x)).collect()
    }
}

impl Posts {
    /// get `preview_url` of all posts
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_p_urls();
    /// }
    /// ```
    pub fn get_p_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.preview_url));
        urls
    }
    /// get `sample_url` of all posts
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_s_urls();
    /// }
    /// ```
    pub fn get_s_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.sample_url));
        urls
    }
    /// get `file_url` of all posts
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_f_urls();
    /// }
    /// ```
    pub fn get_f_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.file_url));
        urls
    }

    /// Get [`MiniPost`] of all posts
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_urls_ext();
    /// }
    /// ```
    #[inline]
    pub fn get_urls_ext(&self) -> MiniPosts {
        self.into()
    }

    /// Get first [`MiniPost`] of all posts
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_url_ext();
    /// }
    /// ```
    #[inline]
    pub fn get_url_ext(&self) -> Option<MiniPost> {
        self.0.first().map(|x| x.into())
    }

    /// get first `preview_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_p_url();
    /// }
    /// ```
    #[inline]
    pub fn get_p_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.preview_url.as_str())
    }
    /// get first `sample_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_s_url();
    /// }
    /// ```
    #[inline]
    pub fn get_s_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.sample_url.as_str())
    }
    /// get first `file_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_f_url();
    /// }
    /// ```
    #[inline]
    pub fn get_f_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.file_url.as_str())
    }

    /// Make ref from [Posts]
    #[inline]
    pub fn data_ref(&self) -> &Vec<Post> {
        &self.0
    }

    /// Take [Vec] from [Posts]
    #[inline]
    pub fn data(self) -> Vec<Post> {
        self.0
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique file_urls (raw urls) as string slices (`&str`).
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_p_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_f_urls(&self, size: usize) -> Vec<&str> {
        random_usize_vec!(self.get_f_urls(), size)
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique preview_urls as string slices (`&str`).
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_p_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_p_urls(&self, size: usize) -> Vec<&str> {
        random_usize_vec!(self.get_p_urls(), size)
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique sample urls as string slices (`&str`).
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_s_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_s_urls(&self, size: usize) -> Vec<&str> {
        random_usize_vec!(self.get_s_urls(), size)
    }

    /// Retrieves a specified number of random unique URLs from the collection of F URLs.
    ///
    /// **Parameters:**
    /// - `size`: The number of random unique URLs to retrieve.
    ///   Must be less than or equal to the total number of F URLs available.
    ///
    /// **Returns:**
    /// - A vector containing `size` random unique samples [MiniPost].
    ///
    /// **Errors:**
    /// - If `size` is greater than the number of available F URLs, an assertion will be triggered,
    ///   and the program will terminate with an error.
    ///
    /// **Example usage:**
    /// ```rust
    /// use shuller::prelude::*;
    ///
    /// async fn get_random_vec_with_links() {
    ///     let binding = R34Params::init().limit(10).download().await.unwrap();
    ///     let result = binding.get_random_s_urls(3);
    ///     println!("{:#?}", result);
    /// }
    /// ```
    ///
    /// **Note:**
    /// This function requires the `rand` feature to be enabled, as it relies on the `random_usize_vec` macro
    /// to select random elements.
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_urls_ext(&self, size: usize) -> MiniPosts {
        use crate::random_usize_vec_cloned;
        let a = self.get_urls_ext().0;
        random_usize_vec_cloned!(&a, size)
    }

    #[cfg(feature = "rand")]
    #[inline]
    pub fn shuffle(&self) -> Posts {
        use crate::random_usize_vec_cloned;
        random_usize_vec_cloned!(self.0, self.0.len())
    }

    /// Just is_empty()
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use uller::JsonDownload;

    use crate::prelude::*;

    #[tokio::test]
    async fn create_make_link_search_with_id() {
        //fishey fishey
        let binding = R34Params::init().id(10542274).download().await.unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result);
        assert!(!result.is_empty())
    }
    #[tokio::test]
    async fn check_error() {
        let binding = R34Params::init().limit(10).download().await.unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result);
        assert!(!result.is_empty());
        let result = binding.get_random_f_urls(3);
        println!("{:#?}", result);
    }
}
