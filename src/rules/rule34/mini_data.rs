use std::fmt::{write, Display};

use super::data::{Post, Posts};

/// Less info struct of [Post]
#[derive(Debug, Clone)]
pub struct MiniPost<'a> {
    id: u64,
    /// raw picture
    /// W - `wight`
    /// H - `hight`
    file_url: &'a str,
    /// W of raw picture (`file_url`)
    width: u64,
    /// H of raw picture (`file_url`)
    height: u64,
    /// tags
    tags: &'a str,
}

impl<'a> MiniPost<'a> {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn file_url(&self) -> &str {
        self.file_url
    }

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }

    pub fn tags(&self) -> &str {
        self.tags
    }
    pub fn tags_vec(&self) -> Vec<&str> {
        self.tags.split(" ").collect()
    }
}

impl<'a> From<MiniPosts<'a>> for Vec<String> {
    fn from(val: MiniPosts<'a>) -> Self {
        val.0.iter().map(|x| format!("{}", x)).collect()
    }
}

impl<'a> From<&'a Post> for MiniPost<'a> {
    fn from(val: &'a Post) -> Self {
        MiniPost {
            id: val.id as u64,
            file_url: &val.file_url,
            width: val.width as u64,
            height: val.height as u64,
            tags: &val.tags,
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
        let a: Vec<MiniPost> = value.data_ref().iter().map(|x| x.into()).collect();
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
// Реализация From для Vec::new()
impl<'a> From<()> for MiniPosts<'a> {
    #[inline]
    fn from(_: ()) -> Self {
        MiniPosts::default()
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

    /// get `file_url` of all posts
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = R34Params::init().download().await.unwrap();
    ///     let result = binding.get_f_urls();
    /// }
    /// ```
    pub fn get_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(x.file_url));
        urls
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
    pub fn get_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.file_url)
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
    pub fn get_random_urls(&self, size: usize) -> Vec<&str> {
        use crate::random_usize_vec;
        random_usize_vec!(self.get_urls(), size)
    }

    /// Shuffle and wrap size
    #[cfg(feature = "rand")]
    #[inline]
    pub fn get_random(&self, size: usize) -> MiniPosts<'a> {
        self.shuffle().0[0..size].to_vec().into()
    }

    /// Shuffle it!
    #[cfg(feature = "rand")]
    #[inline]
    pub fn shuffle(&self) -> MiniPosts<'a> {
        use crate::random_usize_vec_cloned;
        random_usize_vec_cloned!(self.0, self.0.len())
    }

    /// Convert to String in special format:
    /// "id: {}, tags: {:#?}, url: {}"
    #[inline]
    pub fn make_detail_list(&self) -> Vec<String> {
        self.0.iter().map(|x| x.to_string()).collect()
    }
}
impl<'a> AsRef<Vec<MiniPost<'a>>> for MiniPosts<'a> {
    fn as_ref(&self) -> &Vec<MiniPost<'a>> {
        &self.0
    }
}
