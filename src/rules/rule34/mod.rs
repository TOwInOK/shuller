/// Params for `Rule34` Api
pub mod params;
/// This is law of rules!
pub mod rule;

/// Picture structure
///
/// Contain url's of cdn
pub mod data;

/// Macros for fast creating rule using specific pattern
///
/// ### Rule34
/// arguments: `id`; `positive_tag`, `negative_tag`, `limit`, `page`
///
/// ### Example
///
/// ```
/// use shuller::prelude::*;
///
/// let instance = rule34!(vec!["dark", "fish"], vec!["ai_generated"]);
/// let specific_instance_with_id = rule34!(vec![], vec![]; 10542274);
/// let instance_with_id = rule34!(; 10542274);
/// ```
#[macro_export]
macro_rules! rule34 {
    ($positive_tag:expr, $negative_tag:expr, $limit:expr, $page:expr) => {{
        let params = Params::init()
            .limit($limit)
            .page($page)
            .positive_tags($positive_tag)
            .negative_tags($negative_tag)


        Rules::Rule34(params)
    }};
    ($positive_tag:expr, $negative_tag:expr, $limit:expr, $page:expr) => {{
            let params = Params::init()
                .limit($limit)
                .page($page)
                .positive_tags($positive_tag)
                .negative_tags($negative_tag);

            Rules::Rule34(params)
        }};

        ($positive_tag:expr, $negative_tag:expr, $limit:expr) => {{
            let params = Params::init()
                .limit($limit)
                .positive_tags($positive_tag)
                .negative_tags($negative_tag);

            Rules::Rule34(params)
        }};

        ($positive_tag:expr, $negative_tag:expr $(;$id:expr)?) => {{
            let params = Params::init()
                .positive_tags($positive_tag)
                .negative_tags($negative_tag)
                $(.id($id))?;

            Rules::Rule34(params)
        }};
        ($positive_tag:expr) => {{
            let params = Params::init().positive_tags($positive_tag);

            Rules::Rule34(params)
        }};
        (; $($id:expr)?) => {{
            let params = Params::init()
            $(.id($id))?;


            Rules::Rule34(params)
        }
    };
}
