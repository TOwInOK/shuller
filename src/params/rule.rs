use crate::link::link::MakeLink;

/// Rule34 implementation
///
/// # Sample
///
/// ```
/// let instance = T::init()
///     .negative_tags(vec!["ai_generated"])
///     .positive_tags(vec!["anime", "base", "sunglasses"])
///     .limit(1)
///     .page(3)
///     .make_link();
/// ```
pub trait Rule34<'a>
where
    Self: MakeLink,
{
    /// Init params builder
    ///
    /// # Usage
    ///
    /// ```
    /// let instance = T::init();
    /// ```
    fn init() -> Self
    where
        Self: Default,
    {
        Self::default()
    }
    /// # Set positive tags
    ///
    /// # Usage
    ///
    /// ```
    /// let instance = T::init().positive_tags(vec!["molly"]);
    /// ```
    fn positive_tags(self, tags: Vec<&'a str>) -> Self;
    /// # Set negative tags
    ///
    /// # Usage
    ///
    /// ```
    /// let instance = T::init().negative_tags(vec!["ai_generated"]);
    /// ```
    fn negative_tags(self, tags: Vec<&'a str>) -> Self;
    /// # Set limit of links in response
    /// max limit <=1000
    /// # Usage
    ///
    /// ```
    /// let instance = T::init().limit(2);
    /// ```
    fn limit(self, limit: u16) -> Self;
    /// # start page for find pictures
    ///
    /// # Usage
    ///
    /// ```
    /// let instance = T::init().page(20);
    /// ```
    fn page(self, page: u16) -> Self;
}

// Any rules?
