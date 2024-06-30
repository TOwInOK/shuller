use crate::link::link::Link;

use super::rule::Rule34;

#[derive(PartialEq, Debug)]
pub struct Params<'a> {
    /// Default value for api
    // page
    p: &'a str,
    /// Default value for api
    s: &'a str,
    /// Default value for api
    q: &'a str,
    /// Tags, main tool for search
    positive_tags: Vec<&'a str>,
    negative_tags: Vec<&'a str>,
    /// Default value for lib
    json: bool,
    /// Limit of links in response MAX: 1000
    /// By default = 1
    limit: u16,
    /// Page
    /// By default = 1
    // pid
    page: u16,
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
        }
    }
}

impl<'a> Rule34<'a> for Params<'a> {
    /// Init params builder
    fn init() -> Self {
        Self::default()
    }
    /// Set positive tags
    fn positive_tags(mut self, mut tags: Vec<&'a str>) -> Self {
        self.positive_tags.append(&mut tags);
        self
    }
    /// Set negative tags
    fn negative_tags(mut self, mut tags: Vec<&'a str>) -> Self {
        self.negative_tags.append(&mut tags);
        self
    }
    /// Set limit of links in response
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
    fn page(mut self, page: u16) -> Self {
        self.page = page;
        self
    }

    /// Make the link for request
    fn build(&self) -> Link {
        let suppressed_tags = self.tags_suppress();
        let json = self.json_convert();
        let link = format!(
            "https://api.rule34.xxx/index.php?page={}&s={}&q={}&tags={}&json={}&limit={}&pid={}",
            self.p, self.s, self.q, suppressed_tags, json, self.limit, self.page
        );
        Link::init(link)
    }

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
            }
        );
    }
    #[test]
    fn build() {
        let result = Params::init()
            .positive_tags(vec![":)"])
            .negative_tags(vec![":("])
            .limit(5)
            .page(2)
            .build();
        let expected = Link::init(format!(
            "https://api.rule34.xxx/index.php?page={}&s={}&q={}&tags={}&json={}&limit={}&pid={}",
            "dapi", "post", "index", ":) -:(", 1, 5, 2
        ));
        assert_eq!(result, expected)
    }
}
