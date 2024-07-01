use reqwest::Url;

use crate::prelude::*;

/// This is `Rule34`'s rule to create `Link`s
pub mod rule34;

/// Multi build provider
pub enum Rules<'a> {
    Rule34(rule34::params::Params<'a>),
    // other rules or sites
    Other(Box<dyn MakeLink + 'a>),
}

impl MakeLink for Rules<'_> {
    fn make_link(&self) -> Result<Url> {
        match &self {
            Rules::Rule34(x) => x.make_link(),
            Rules::Other(x) => x.make_link(),
        }
    }
}

impl Search for Rules<'_> {}
