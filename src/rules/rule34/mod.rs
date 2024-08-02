/// Picture structure
///
/// Contain url's of cdn
pub mod data;
/// Params for `Rule34` Api
pub mod params;
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
///  * **R** Random -> Generate with random id [usize].
///    - Has 3 varants
///        - **R** Generate [crate::rules::rule34::params::R34Params]
///        - **D** Generate [crate::rules::rule34::data::Posts]
///        - **U** Generate [uller::Url]
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
///  * **R** Random
///     ```rust
///     use shuller::prelude::*;
///     async fn r() {
///         let params = R34!(R;);
///         let url = R34!(R; D).unwrap();
///         let posts = R34!(R; U);
///     }
///     ```
#[macro_export]
macro_rules! R34 {
    // Normal
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
    // Download
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
    // Url
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
    // Random

    // Random Params
    (R;) => {
        R34Params::init()
            .id(random_usize!(10900000))
    };
    // Random Post
    (R; D) => {
        R34Params::init()
            .id(random_usize!(10900000))
            .download().await
    };
    // Random Url
    (R; U) => {
        R34Params::init()
            .id(random_usize!(10900000))
            .url_generate()
    }
}

/// Generate random [usize] number in customizable max range
///
/// **in** => [usize] value
///
/// **out** => [usize]
#[cfg(feature = "rand")]
#[macro_export]
macro_rules! random_usize {
    ($max:expr) => {{
        rand::thread_rng().gen_range(usize::MIN..$max)
    }};
}
