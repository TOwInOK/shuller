use crate::link::make_link::MakeLink;

/// Rule34 implementation
pub trait Rule34<'a>
where
    Self: MakeLink,
{
    /// Init params builder
    fn init() -> Self
    where
        Self: Default,
    {
        Self::default()
    }

    /// # Set positive tags
    ///
    fn positive_tags(self, tags: Vec<&'a str>) -> Self;

    /// # Set negative tags
    fn negative_tags(self, tags: Vec<&'a str>) -> Self;

    /// # Set limit of links in response
    /// max limit <=1000
    fn limit(self, limit: u16) -> Self;

    /// # Set id of post
    fn id(self, id: usize) -> Self;

    /// # start page for find pictures
    fn page(self, page: u16) -> Self;
}

// Any rules?
