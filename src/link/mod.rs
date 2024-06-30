/// Trait for making `Link`
pub mod make_link;
/// Picture structure
///
/// Contain url's of cdn
pub mod post;

use serde::Deserialize;

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
    /// use shuller::prelude::*;
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
    pub async fn search<T>(&self) -> Result<T, reqwest::Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let posts: T = reqwest::get(self.url()).await?.json().await?;
        Ok(posts)
    }
}
