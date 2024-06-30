use serde::Deserialize;
use serde::Serialize;

/// # List of Posts
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Posts(Vec<Post>);

/// # Post structure
///
/// * Post in our context of image structure which contain some urls
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Posts {
    /// get `preview_url` of all posts
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = Params::init().make_link().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().make_link().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().make_link().search().await.unwrap();
    ///     let result = binding.get_f_urls();
    /// }
    /// ```
    pub fn get_f_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.file_url));
        urls
    }

    /// get first `preview_url`
    /// ```
    /// use shuller::prelude::*;
    ///
    /// async fn dwl() {
    ///     let binding: Posts = Params::init().make_link().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().make_link().search().await.unwrap();
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
    ///     let binding: Posts = Params::init().make_link().search().await.unwrap();
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
        let binding: Posts = Params::init().make_link().search().await.unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result)
    }

    #[tokio::test]
    async fn create_make_link_search_with_id() {
        //fishey fishey
        let binding: Posts = Params::init()
            .id(10542274)
            .make_link()
            .search()
            .await
            .unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result)
    }
}
