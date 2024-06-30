use crate::link::{make_link::MakeLink, Link};

use super::rule::Rule34;

/// Rule 34 params
///
/// # Example
///
/// ```
/// use shuller::prelude::*;
///
/// let instance = Params::init()
///     .negative_tags(vec!["ai_generated"])
///     .positive_tags(vec!["anime", "base", "sunglasses"])
///     .limit(1)
///     .page(3)
///     .make_link();
/// ```
/// `instance` is { url: `https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&tags=anime base sunglasses -ai_generated&json=1&limit=5&pid=2` }
#[derive(PartialEq, Debug)]
pub struct Params<'a> {
    /// Default value for api
    // page
    p: &'a str,
    /// Default value for api
    s: &'a str,
    /// Default value for api
    q: &'a str,
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
    /// id of post
    pub id: Option<usize>,
}

impl Default for Params<'_> {
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

impl<'a> Rule34<'a> for Params<'a> {
    /// Set positive tags
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = Params::init()
    ///     .positive_tags(vec!["molly"]);
    ///
    /// ```
    fn positive_tags(mut self, mut tags: Vec<&'a str>) -> Self {
        self.positive_tags.append(&mut tags);
        self
    }
    /// Set negative tags
    ///
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = Params::init()
    ///     .negative_tags(vec!["ai_generated"]);
    ///
    /// ```
    fn negative_tags(mut self, mut tags: Vec<&'a str>) -> Self {
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
    /// let result = Params::init()
    ///     .limit(2);
    ///
    /// ```
    fn limit(mut self, limit: u16) -> Self {
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
    /// let result = Params::init()
    ///     .page(2);
    ///
    /// ```
    fn page(mut self, page: u16) -> Self {
        self.page = page;
        self
    }

    /// Set id of post
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = Params::init()
    ///     .id(2);
    ///
    /// ```
    fn id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }
}

impl MakeLink for Params<'_> {
    /// # Build
    ///
    /// Creates a `Link` for making request.
    ///
    /// Need to pass `&Params`
    ///
    /// #### Simple example
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = Params::init().make_link();
    ///
    /// ```
    /// #### Basic example
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = Params::init()
    ///     .positive_tags(vec![":)"])
    ///     .negative_tags(vec![":("])
    ///     .limit(5)
    ///     .page(2)
    ///     .make_link();
    ///
    /// ```
    /// #### Example with id
    ///
    /// We may not use other options since we use the specific id
    /// ```
    /// use shuller::prelude::*;
    ///
    /// let result = Params::init()
    ///     .id(99999)
    ///     .make_link();
    ///
    /// ```
    fn make_link(&self) -> Link {
        if let Some(id) = self.id {
            return Link::init(format!(
                "https://api.rule34.xxx/index.php?page={}&s={}&q={}&tags={}&json={}&limit={}&pid={}&id={}",
                self.p,
                self.s,
                self.q,
                self.tags_suppress(),
                self.json_convert(),
                self.limit,
                self.page,
                id
            ));
        }

        Link::init(format!(
            "https://api.rule34.xxx/index.php?page={}&s={}&q={}&tags={}&json={}&limit={}&pid={}",
            self.p,
            self.s,
            self.q,
            self.tags_suppress(),
            self.json_convert(),
            self.limit,
            self.page
        ))
    }
}

impl Params<'_> {
    /// Combination 2 lists of tags in one for query
    fn tags_suppress(&self) -> String {
        let pt = self.positive_tags.join(" ");
        let nt: Vec<String> = self
            .negative_tags
            .iter()
            .map(|x| format!("-{}", x))
            .collect();
        let nt = nt.join(" ");
        format!("{} {}", pt, nt)
    }

    fn json_convert(&self) -> u8 {
        if self.json {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn init() {
        let result = Params::init();
        assert_eq!(
            result,
            Params {
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
        let result = Params::init().id(2);
        assert_eq!(
            result,
            Params {
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
        let result = Params::init().positive_tags(vec!["test"]);
        assert_eq!(
            result,
            Params {
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
        let result = Params::init().negative_tags(vec!["test"]);
        assert_eq!(
            result,
            Params {
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
        let result = Params::init().limit(30);
        assert_eq!(
            result,
            Params {
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
        let result = Params::init().limit(1001);
        assert_eq!(
            result,
            Params {
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
        let result = Params::init()
            .negative_tags(vec!["test"])
            .positive_tags(vec!["test"])
            .tags_suppress();
        assert_eq!(result, format!("test -test"))
    }
    #[test]
    fn json_convert() {
        let result = Params::init().json_convert();
        assert_eq!(result, 1)
    }
    #[test]
    fn page() {
        let result = Params::init().page(30);
        assert_eq!(
            result,
            Params {
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
        let result = Params::init()
            .negative_tags(vec!["ai_generated"])
            .positive_tags(vec!["anime", "base", "sunglasses"])
            .limit(5)
            .page(2)
            .make_link();
        let expected = Link::init(format!(
            "https://api.rule34.xxx/index.php?page={}&s={}&q={}&tags={}&json={}&limit={}&pid={}",
            "dapi", "post", "index", "anime base sunglasses -ai_generated", 1, 5, 2
        ));
        assert_eq!(result, expected)
    }
}
