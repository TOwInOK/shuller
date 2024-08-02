//! # Shuller
//!
//! Crate for making links by something structures
//! and
//! Getting some data from links
//!
//! # Example
//!
//! ```
//! use shuller::prelude::*;
//!
//! async fn example() {
//!     let instance: Posts = R34Params::init()
//!         .positive_tags(vec!["dark", "fish"])
//!         .negative_tags(vec!["ai_generated"])
//!         .limit(3)
//!         .download()
//!         .await
//!         .unwrap();
//!         println!("{:#?}", instance.get_urls_ext())
//! }
//! ```
//!
//! # Example via macro
//!
//! ```
//! use shuller::prelude::*;
//!
//! async fn macro_normal() {
//!     let instance = R34!(
//!         p = vec!["dark", "fish"],
//!         n = vec!["ai_generated"],
//!         limit = 2,
//!         page = 2
//!     )
//!     .download()
//!     .await
//!     .unwrap();
//!     assert!(instance.get_f_urls().len() == 2)
//! }
//! async fn macro_download() {
//!     let instance = R34!(D;
//!         p = vec!["dark", "fish"],
//!         n = vec!["ai_generated"],
//!         limit = 2,
//!         page = 2
//!     )
//!     .unwrap();
//!     assert!(instance.get_f_urls().len() == 2)
//! }
//! async fn macro_url() {
//!     let instance = R34!(U;
//!         p = vec!["dark", "fish"],
//!         n = vec!["ai_generated"],
//!         limit = 2,
//!         page = 2
//!     );
//!     assert!(instance.is_special())
//! }
//! ```
/// Rules for making links
///
/// If you want to create you own rules
/// just add Traits [uller::MakeLink] and [uller::JsonDownload] from uller crate
/// and for output own structure you need create own struct which implemented [serde::Deserialize]
pub mod rules;

/// Just all that you need
#[allow(unused)]
pub mod prelude {
    #[cfg(feature = "rand")]
    pub use crate::random_usize;

    pub use crate::rules::rule34::data::{MiniPost, Post, Posts};
    pub use crate::rules::rule34::params::R34Params;
    pub use crate::{tag_suppress, toggler, R34};

    #[cfg(feature = "rand")]
    pub use rand::prelude::ThreadRng;
    #[cfg(feature = "rand")]
    pub use rand::Rng;

    pub use uller::{JsonDownload, MakeLink};
}
