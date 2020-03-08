use url::{Url, ParseError};

#[derive(PartialEq, Eq, Hash)]
pub struct Website {
    url : Url
}

impl Website {
    pub fn from_path(path : &str) -> Result<Self, ParseError> {
        Website {
            url: Url::parse(path)?
        }.into();
    }
}