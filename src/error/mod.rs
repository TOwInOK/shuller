use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Fail to parse url: {0}")]
    ParseUrl(#[from] url::ParseError),
    // Any(#[from] Box<dyn std::error::Error + Send>),
}

pub type Result<T> = std::result::Result<T, Error>;
