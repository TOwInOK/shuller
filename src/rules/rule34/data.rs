use serde::Deserialize;

/// # List of [`Posts`]
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Posts(Vec<Post>);

/// # [`Post`] structure
///
/// * Post in our context of image structure which contain some urls
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
    /// Home many people come on this image?
    pub score: i64,
    /// Tags of picture
    pub tags: String,
    /// Used for translation (in my case)
    pub has_notes: bool,
    // pub comment_count: i64,
}

/// Less info struct of [`Post`]
#[derive(Debug)]
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
    ///     let binding: Posts = Params::init().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().search().await.unwrap();
    ///     let result = binding.get_urls_ext();
    /// }
    /// ```
    pub fn get_urls_ext(&self) -> Vec<MiniPost> {
        self.0.iter().map(|x| x.into()).collect()
    }

    /// Get first [`MiniPost`] of all posts
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = Params::init().search().await.unwrap();
    ///     let result = binding.get_url_ext();
    /// }
    /// ```
    pub fn get_url_ext(&self) -> Option<MiniPost> {
        self.0.first().map(|x| x.into())
    }

    /// get first `preview_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = Params::init().search().await.unwrap();
    ///     let result = binding.get_p_url();
    /// }
    /// ```
    pub fn get_p_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.preview_url.as_str())
    }
    /// get first `sample_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = Params::init().search().await.unwrap();
    ///     let result = binding.get_s_url();
    /// }
    /// ```
    pub fn get_s_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.sample_url.as_str())
    }
    /// get first `file_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = Params::init().search().await.unwrap();
    ///     let result = binding.get_f_url();
    /// }
    /// ```
    pub fn get_f_url(&self) -> Option<&str> {
        self.0.first().map(|x| x.file_url.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[tokio::test]
    async fn create_make_link_search() {
        let binding: Posts = Params::init().search().await.unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result)
    }

    #[tokio::test]
    async fn create_make_link_search_with_id() {
        //fishey fishey
        let binding: Posts = Params::init().id(10542274).search().await.unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result)
    }
}
