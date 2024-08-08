use crate::random_usize;
use crate::rules::rule34::data::Posts;
use crate::tag_suppress;
use crate::toggler;
use async_trait::async_trait;
use uller::{JsonDownload, MakeLink, Url};

/// Rule 34 params
///
/// # Example
///
/// ```
/// use shuller::prelude::*;
///
/// let instance = R34Params::init()
///     .negative_tags(vec!["ai_generated"])
///     .positive_tags(vec!["anime", "base", "sunglasses"])
///     .limit(1)
///     .page(3)
///     .url_generate();
/// ```
/// **instance** is { url: `https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&tags=anime base sunglasses -ai_generated&json=1&limit=5&pid=2` }
#[derive(PartialEq, Debug)]
pub struct R34Params<'a> {
    /// Default value for api
    // page
    p: &'static str,
    /// Default value for api
    s: &'static str,
    /// Default value for api
    q: &'static str,
    /// Positive tags
    pub positive_tags: Vec<&'a str>,
    /// Negative tags (`"-"` sets automaticly)
    pub negative_tags: Vec<&'a str>,
    /// Default value for lib
    json: bool,
    /// Limit of links in response MAX: 1000
    /// By default = 1
    pub limit: u16,
    /// Start page to search (1 page = 10 pictures)
    /// By default = 1
    // pid
    pub page: u16,
    // id of post
    pub id: Option<usize>,
}

impl MakeLink for R34Params<'_> {
    fn url_generate(&self) -> Url {
        let url = "https://api.rule34.xxx/index.php";
        let tags = tag_suppress!(self.positive_tags, self.negative_tags);
        if let Some(id) = self.id {
            return Url::parse_with_params(
                url,
                &[
                    ("page", self.p),
                    ("s", self.s),
                    ("q", self.q),
                    ("tags", &tags),
                    ("json", toggler!(self.json)),
                    ("limit", self.limit.to_string().as_ref()),
                    ("pid", self.page.to_string().as_ref()),
                    ("id", id.to_string().as_ref()),
                ],
            )
            .expect("Failed to parse URL with params");
        }
        Url::parse_with_params(
            url,
            &[
                ("page", self.p),
                ("s", self.s),
                ("q", self.q),
                ("tags", &tags),
                ("json", toggler!(self.json)),
                ("limit", self.limit.to_string().as_ref()),
                ("pid", self.page.to_string().as_ref()),
            ],
        )
        .expect("Failed to parse URL with params")
    }
}

impl Default for R34Params<'_> {
    fn default() -> Self {
        Self {
            // page
            p: "dapi",
            s: "post",
            q: "index",
            positive_tags: vec![],
            negative_tags: vec![],
            json: true,
            limit: 1,
            // pid
            page: 1,
            id: None,
        }
    }
}

impl<'a> R34Params<'a> {
    /// Init params
    #[inline]
    pub fn init() -> Self {
        Self::default()
    }
    /// Set positive tags
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = R34Params::init()
    ///     .positive_tags(vec!["molly"]);
    ///
    /// ```
    #[inline]
    pub fn positive_tags(mut self, mut tags: Vec<&'a str>) -> Self {
        self.positive_tags.append(&mut tags);
        self
    }
    /// Set negative tags
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = R34Params::init()
    ///     .negative_tags(vec!["ai_generated"]);
    ///
    /// ```
    #[inline]
    pub fn negative_tags(mut self, mut tags: Vec<&'a str>) -> Self {
        self.negative_tags.append(&mut tags);
        self
    }
    /// Set limit of links in response
    ///
    /// max limit <=1000
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = R34Params::init()
    ///     .limit(2);
    ///
    /// ```
    #[inline]
    pub fn limit(mut self, limit: u16) -> Self {
        if limit > 1000 {
            eprintln!("Limit is greater then expected:");
            eprintln!("Expected: any numbers <= 1000, got: {0}", limit);
            eprintln!("Set limit at 1000");

            self.limit = 1000;
        } else {
            self.limit = limit;
        }
        self
    }
    /// start page for find pictures
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = R34Params::init()
    ///     .page(2);
    ///
    /// ```
    #[inline]
    pub fn page(mut self, page: u16) -> Self {
        self.page = page;
        self
    }

    /// Set id of post
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = R34Params::init()
    ///     .id(2);
    ///
    /// ```
    #[inline]
    pub fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    /// Set random id for post
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = R34Params::init()
    ///     .gen_id();
    ///
    /// ```
    pub fn gen_id(mut self) -> Self {
        const MAX_RANDOM_ID: usize = 10900000;
        self.id = Some(random_usize!(MAX_RANDOM_ID));
        self
    }
}

#[async_trait]
impl JsonDownload<Posts> for R34Params<'_> {}

#[cfg(test)]
mod tests {

    // use super::*;

    use uller::MakeLink;
    use url::Url;

    use crate::prelude::R34Params;

    #[test]
    fn init() {
        let result = R34Params::init();
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec![],
                negative_tags: vec![],
                json: true,
                limit: 1,
                page: 1,
                id: None
            }
        );
    }

    #[test]
    fn init_with_id() {
        let result = R34Params::init().id(2);
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec![],
                negative_tags: vec![],
                json: true,
                limit: 1,
                page: 1,
                id: Some(2)
            }
        );
    }

    #[test]
    fn positive_params() {
        let result = R34Params::init().positive_tags(vec!["test"]);
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec!["test"],
                negative_tags: vec![],
                json: true,
                limit: 1,
                page: 1,
                id: None
            }
        );
    }
    #[test]
    fn negative_params() {
        let result = R34Params::init().negative_tags(vec!["test"]);
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec![],
                negative_tags: vec!["test"],
                json: true,
                limit: 1,
                page: 1,
                id: None
            }
        );
    }
    #[test]
    fn limit() {
        let result = R34Params::init().limit(30);
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec![],
                negative_tags: vec![],
                json: true,
                limit: 30,
                page: 1,
                id: None
            }
        );
    }
    #[test]
    fn limit_greaten() {
        let result = R34Params::init().limit(1001);
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec![],
                negative_tags: vec![],
                json: true,
                limit: 1000,
                page: 1,
                id: None
            }
        );
    }
    #[test]
    fn tag_suppress() {
        use crate::tag_suppress;
        let result = R34Params::init()
            .negative_tags(vec!["test"])
            .positive_tags(vec!["test"]);

        assert_eq!(
            tag_suppress!(result.positive_tags, result.negative_tags),
            format!("test -test")
        )
    }

    #[test]
    fn page() {
        let result = R34Params::init().page(30);
        assert_eq!(
            result,
            R34Params {
                p: "dapi",
                s: "post",
                q: "index",
                positive_tags: vec![],
                negative_tags: vec![],
                json: true,
                limit: 1,
                page: 30,
                id: None
            }
        );
    }
    #[test]
    fn build() {
        let result = R34Params::init()
            .negative_tags(vec!["ai_generated"])
            .positive_tags(vec!["anime", "base", "sunglasses"])
            .limit(5)
            .page(2)
            .url_generate();
        let expected = Url::parse_with_params(
            "https://api.rule34.xxx/index.php?",
            [
                ("page", "dapi"),
                ("s", "post"),
                ("q", "index"),
                ("tags", "anime base sunglasses -ai_generated"),
                ("json", "1"),
                ("limit", "5"),
                ("pid", "2"),
            ],
        )
        .unwrap();

        assert_eq!(result, expected)
    }

    #[tokio::test]
    async fn create_many() {
        let mut instances = vec![];
        for _ in 0..=5 {
            instances.push(R34Params::init().gen_id().url_generate().to_string())
        }
        for item in instances {
            // let x = item.join().unwrap();
            println!("{}", item)
        }
    }
}
