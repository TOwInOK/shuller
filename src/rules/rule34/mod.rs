/// Params for `Rule34` Api
pub mod params;

/// Picture structure
///
/// Contain url's of cdn
pub mod data;
// pub use crate::rules::rule34::params::R34Params;
// pub use crate::rules::rule34::data::Posts;
/// Macros for fast creating rule34 params by using specific pattern
///
/// ### Rule34
/// #### arguments:
///  * p = `positive_tag`::<[`Vec<String>`]>
///     - default = []
///     - optional
///  * n = `negative_tag`::<[`Vec<String>`]>
///     - default = []
///     - optional
///  * limit = `limit`::<[u64]>
///     - default = 1
///     - optional
///  * page = `page`::<[u64]>
///     - default = 1
///     - optional
///
/// **or**
///
/// #### argument:
///  * `id` [usize]::<[usize]>
///     - default = [None]
///     - optional
///
/// #### This macro has 3 mode of work
///  * Normal -> Give you [crate::rules::rule34::params::R34Params]
///  * **D** Download -> Give you output [crate::rules::rule34::data::Posts] of [crate::rules::rule34::params::R34Params] url generation
///  * **U** Url -> Just generate [uller::Url] by [crate::rules::rule34::params::R34Params]
/// ### Examples
///  * Normal
///    ```rust
///    use shuller::prelude::*;
///    fn normal() {
///        let instance = R34!(p = vec!["dark", "fish"], n = vec!["ai_generated"], limit = 1, page = 2);
///        let instance_with_id = R34!(id = 222222);
///    }
///    ```
///  * **D** Download
///     ```rust
///     use shuller::prelude::*;
///     async fn d() {
///         let instance = R34!(D; p = vec!["dark", "fish"], n = vec!["ai_generated"], limit = 1, page = 2);
///         let instance_with_id = R34!(D; id = 222222);
///     }
///     ```
///  * **U** Url
///     ```rust
///     use shuller::prelude::*;
///     async fn u() {
///         let instance = R34!(U; p = vec!["dark", "fish"], n = vec!["ai_generated"], limit = 1, page = 2);
///         let instance_with_id = R34!(U; id = 222222);
///     }
///     ```
#[macro_export]
macro_rules! R34 {
    ($(p = $positive_tags:expr)? $(,n = $negative_tags:expr)? $(,limit = $limit:expr)? $(,page = $page:expr)?) => {{
        R34Params::init()
            $(.limit($limit))?
            $(.positive_tags($positive_tags))?
            $(.negative_tags($negative_tags))?
    }};
    ($(id = $id:expr)?) => {{
        R34Params::init()
            $(.id($id))?
    }};
    (D; $(p = $positive_tags:expr)? $(,n = $negative_tags:expr)? $(,limit = $limit:expr)? $(,page = $page:expr)?) => {{
        R34Params::init()
            $(.limit($limit))?
            $(.positive_tags($positive_tags))?
            $(.negative_tags($negative_tags))?
            .download().await
    }};
    (D; $(id = $id:expr)?) => {{
        R34Params::init()
            $(.id($id))?.download().await
    }};
    (U; $(p = $positive_tags:expr)? $(,n = $negative_tags:expr)? $(,limit = $limit:expr)? $(,page = $page:expr)?) => {{
        R34Params::init()
            $(.limit($limit))?
            $(.positive_tags($positive_tags))?
            $(.negative_tags($negative_tags))?
            .url_generate()
    }};
    (U; $(id = $id:expr)?) => {{
        R34Params::init()
            $(.id($id))?
            .url_generate()
    }};
}
