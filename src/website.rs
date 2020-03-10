use url::{Url, ParseError};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Website {
    url : Url
}

impl Website {
    pub fn from_path(path : &str) -> Result<Self, ParseError> {
        Ok(Website {
            url: Url::parse(path)?
        })
    }
    pub fn get_url(&self) -> &Url {
        &self.url
    }
}