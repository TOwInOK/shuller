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
    pub fn get_urls_ext(&self) -> Vec<MiniPost> {
        self.0.iter().map(|x| x.into()).collect()
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

    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random_urls(&self, size: usize) -> Vec<MiniPost> {
        use crate::random_usize_vec_cloned;

        random_usize_vec_cloned!(self.get_urls_ext(), size)
    }

    /// Just is_empty()
    #[inline(always)]
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
