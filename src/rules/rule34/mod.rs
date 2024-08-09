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
            .gen_id()
    };
    // Random Post
    (R; D) => {
        R34Params::init()
            .gen_id()
            .download().await
    };
    // Random Url
    (R; U) => {
        R34Params::init()
            .gen_id()
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
        use std::time::SystemTime;
        use tinyrand::{Rand, Seeded};
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut rand = tinyrand::StdRand::seed(((time >> 64) ^ time) as u64);
        rand.next_lim_usize($max)
    }};
}

/// Generates a vector of random unique elements from a given vector.
///
/// **Parameters:**
/// - `$vec`: The vector from which elements will be selected.
/// - `$len`: The number of unique elements to return.
///   Must be less than or equal to the length of `$vec`.
///
/// **Returns:**
/// - A vector containing `$len` random unique elements from `$vec`.
/// - If `$len` is equal to the length of `$vec`, a randomly shuffled version of `$vec` is returned.
///
/// **Errors:**
/// - If `$len` is greater than the length of `$vec`, an `assert!` will be triggered, and the program will terminate with an error.
/// - If `$len` is 0 or `$vec` is empty, an empty vector will be returned.
///
/// **Note:**
/// This macro requires the `rand` feature to be enabled.
///
/// **Note:**
/// If `$len` same `$vec` len it return random sorted vec.
#[cfg(feature = "rand")]
#[macro_export]
macro_rules! random_usize_vec {
    ($vec:expr, $len:expr) => {{
        use $crate::random_usize;
        assert!($len <= $vec.len(), "given len is more than max len of vec");
        if $len == 0 || $vec.len() == 0 {
            return vec![];
        }

        let mut temp = vec![];
        let mut indices = std::collections::HashSet::new();

        while indices.len() < $len {
            let index = random_usize!($vec.len());
            indices.insert(index);
        }

        for index in indices {
            temp.push($vec[index]);
        }

        temp
    }};
}

/// Same [random_usize_vec] with clone
#[cfg(feature = "rand")]
#[macro_export]
macro_rules! random_usize_vec_cloned {
    ($vec:expr, $len:expr) => {{
        use $crate::random_usize;
        assert!($len <= $vec.len(), "given len is more than max len of vec");
        if $len == 0 || $vec.len() == 0 {
            return vec![].into();
        }

        let mut temp = vec![];
        let mut indices = std::collections::HashSet::new();

        while indices.len() < $len {
            let index = random_usize!($vec.len());
            indices.insert(index);
        }

        for index in indices {
            temp.push($vec[index].clone());
        }

        temp.into()
    }};
}
