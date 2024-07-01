//! # Shuller
//!
//! Crate for making links by something structures
//! and
//! Getting some data from links
//!
//!
//! # Example via macro
//!
//! ```
//! use shuller::prelude::*;
//!
//! async fn example() {
//! let instance: Posts = rule34!(vec!["dark", "fish"], vec!["ai_generated"])
//!         .search()
//!         .await
//!         .unwrap();
//!     println!("{:#?}", instance.get_s_url());
//! }
//! ```
//!
//! # Example
//!
//! ```
//! use shuller::prelude::*;
//!
//! async fn example() {
//!     let instance: Posts = Params::init()
//!         .positive_tags(vec!["dark", "fish"])
//!         .negative_tags(vec!["ai_generated"])
//!         .search()
//!         .await
//!         .unwrap();
//!     println!("{:#?}", instance.get_urls_ext())
//! }
//! ```
//!
//! # Example via enum
//!
//! ```
//! use shuller::prelude::*;
//!
//! async fn example() {
//!     let instance: Posts = Rules::Rule34(Params::init())
//!         .search()
//!         .await
//!         .unwrap();
//!     println!("{:#?}", instance.get_s_url());
//! }

/// # Allow us to make links from any structures
///
/// # Examples
///
/// ```
/// use shuller::prelude::*;
///
/// async fn print_posts() {
///     let posts: Result<Posts> = Params::init().search().await;
///     match posts {
///         Ok(posts) => {
///             println!("Posts: {:#?}", posts.get_s_urls());
///         }
///         Err(error) => {
///             eprintln!("Error: {}", error);
///         }
///     }
/// }
/// ```
///
pub mod link;
/// Rules for making links
///
/// If you want to create you own rules
/// just add Traits [`crate::link::make_link::MakeLink`] and [`crate::link::Search`]
/// and for output own structure you need create own struct which implemented [`serde::de`]
/// also you can check as example [`crate::rules::rule34::data`] and example in [`crate::link::Search`]
pub mod rules;

/// Errors for Shuller
pub mod error;

/// Very cool feature
#[allow(unused)]
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::link::{make_link::MakeLink, Search};
    pub use crate::rule34;
    pub use crate::rules::rule34::data::MiniPost;
    pub use crate::rules::rule34::data::Post;
    pub use crate::rules::rule34::data::Posts;
    pub use crate::rules::rule34::params::Params;
    pub use crate::rules::rule34::rule::Rule34;
    pub use crate::rules::Rules;
}
