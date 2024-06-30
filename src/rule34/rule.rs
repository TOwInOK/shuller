use crate::link::link::Link;

pub trait Rule34<'a> {
    /// Init params builder
    fn init() -> Self;
    /// Set positive tags
    fn positive_tags(self, tags: Vec<&'a str>) -> Self;
    /// Set negative tags
    fn negative_tags(self, tags: Vec<&'a str>) -> Self;
    /// Set limit of links in response
    fn limit(self, limit: u16) -> Self;
    /// start page for find pictures
    fn page(self, page: u16) -> Self;

    /// Make the link for request
    fn build(&self) -> Link;

    /// Combination 2 lists of tags in one for query
    fn tags_suppress(&self) -> String;

    fn json_convert(&self) -> u8;
}
