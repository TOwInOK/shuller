use reqwest::Url;

use crate::error::Result;

/// # Build
///
/// Creates a `Link` for making request.
pub trait MakeLink
where
    Self: Send + Sync,
{
    fn make_link(&self) -> Result<Url>;
}
