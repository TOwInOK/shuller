use super::Link;

/// # Build
///
/// Creates a `Link` for making request.
pub trait MakeLink {
    fn make_link(&self) -> Link;
}
