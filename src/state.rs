pub struct Params<'a> {
    /// Default value for api
    // page
    p: &'a str,
    /// Default value for api
    s: &'a str,
    /// Default value for api
    q: &'a str,
    /// Tags, main tool for search
    positive_tags: Vec<String>,
    negative_tags: Vec<String>,
    /// Default value for lib
    json: bool,
    /// Limit of links in output request MAX: 1000
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

impl<'a> Params<'a> {
    pub fn init() -> Self {
        Self::default()
    }

    pub fn positive_tags(self) -> Self {
        todo!()
    }
    pub fn negative_tags(self) -> Self {
        todo!()
    }
    pub fn limit(self) -> Self {
        todo!()
    }
    pub fn page(self) -> Self {
        todo!()
    }

    pub fn build(&self) -> String {
        todo!()
    }

    fn tags_suppress(&self) -> &[&str] {
        todo!()
    }
}
