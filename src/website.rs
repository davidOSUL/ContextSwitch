use url::{Url, ParseError};
use serde::{Serialize, Deserialize};

struct Url2 {

}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Website {
    #[serde(with = "url_serde")]
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