/// Search for all posts by the link(url)
///
/// # Example
///
/// ```
///
/// use crate::shuller::link::{link::MakeLink, post::Posts};
/// use shuller::link::link::Link;
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
pub mod link;
/// get `preview_url` of all posts
///
/// # Example
/// ```
/// let binding = Params::init().make_link().search().await.unwrap();
/// let result = binding.get_f_urls();
/// println!("{:#?}", result)
/// ```
pub mod post;
