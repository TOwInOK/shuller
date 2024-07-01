/// Trait for making `Link`
pub mod make_link;

use async_trait::async_trait;
use make_link::MakeLink;
use serde::Deserialize;

use crate::error::Result;

/// # Trait for allowing searching
///
/// Example implementation
/// ```
/// use shuller::prelude::*;
/// use url::Url;
/// use serde::Deserialize;
/// // or use reqwest::Url;
///
/// #[derive(Default)]
/// struct MySearch {}
/// #[derive(Deserialize, Debug)]
/// struct MySearchOutput {}
///
/// impl MakeLink for MySearch {
///     fn make_link(&self) -> Result<Url> {
///         Ok(Url::parse("https://example.com")?)
///     }
/// }
/// impl Search for MySearch {}
///
/// async fn example() {
///     let instance: MySearchOutput = MySearch::default()
///         .search()
///         .await
///         .unwrap();
///     println!("{:#?}", instance)
/// }
/// ```
#[async_trait]
pub trait Search {
    async fn search<T>(&self) -> Result<T>
    where
        for<'de> T: Deserialize<'de>,
        Self: MakeLink + Send + Sync,
    {
        let items: T = reqwest::get(self.make_link()?).await?.json().await?;
        Ok(items)
    }
}
