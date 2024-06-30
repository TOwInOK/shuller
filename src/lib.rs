/// # Links & Posts
/// Search for all posts by the link(url)
///
/// # Examples
///
/// * get `preview_url` of all posts
///
/// ```
/// use shuller::prelude::*;
///
/// async fn print_posts() {
///     let posts: Result<Posts, reqwest::Error> = Link::init("CoolUrl".to_string()).search().await;
///     match posts {
///         Ok(posts) => {
///             println!("Posts: {:#?}", posts);
///         }
///         Err(error) => {
///             eprintln!("Error: {}", error);
///         }
///     }
/// }
///```
///
/// # Example
///
/// * get `preview_url` of one post
///
/// ```
/// use shuller::prelude::*;
///
/// async fn sample() {
///     let binding: Posts = Params::init().make_link().search().await.unwrap();
///     let result = binding.get_f_urls();
///     println!("{:#?}", result)
/// }
/// ```
pub mod link;
/// Rules for making links
///
/// If you want to create you own rules
/// just add Trait [`crate::link::make_link::MakeLink`]
/// and for output structure you can create own struct which implemented [`serde::de`]
/// as example see [`crate::link::Link::search`]
/// also you can check [`crate::rules::rule34::data`]
pub mod rules;

/// Very cool feature
#[allow(unused)]
pub mod prelude {
    pub use crate::link::make_link::MakeLink;
    pub use crate::link::Link;
    pub use crate::rules::rule34::data::MiniPost;
    pub use crate::rules::rule34::data::Post;
    pub use crate::rules::rule34::data::Posts;
    pub use crate::rules::rule34::params::Params;
    pub use crate::rules::rule34::rule::Rule34;
}
