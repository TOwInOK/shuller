use super::post::Posts;

/// Just url
#[derive(Debug, PartialEq)]
pub struct Link {
    url: String,
}

impl Link {
    /// create instance on Link
    pub fn init(url: String) -> Self {
        Self { url }
    }

    fn url(&self) -> &str {
        &self.url
    }

    /// Search for all posts by the link(url)
    ///
    /// # Example
    ///
    /// ```
    /// use shuller::params::{params::Params, rule::Rule34};
    /// use crate::shuller::link::{link::MakeLink, post::Posts};
    ///
    /// async fn print_posts() {
    ///     let posts: Result<Posts, reqwest::Error> = Params::init().make_link().search().await;
    ///     match posts {
    ///         Ok(posts) => {
    ///             println!("Posts: {:#?}", posts);
    ///         }
    ///         Err(error) => {
    ///             eprintln!("Error: {}", error);
    ///         }
    ///     }
    /// }

    /// ```
    pub async fn search(&self) -> Result<Posts, reqwest::Error> {
        let posts: Posts = reqwest::get(self.url()).await?.json().await?;
        Ok(posts)
    }
}

/// # Build
///
/// Creates a `Link` for making request.
pub trait MakeLink {
    fn make_link(&self) -> Link;
}
