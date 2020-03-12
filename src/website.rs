use url::ParseError;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Website {
    //url: Url,
    url: String,
}

impl Website {
    pub fn from_path(path: &str) -> Result<Self, ParseError> {
        //        Ok(Website {
        //            url: Url::parse(path)?,
        //        })
        Ok(Website {
            url: path.to_owned(),
        })
    }
    pub fn get_url_str(&self) -> &str {
        &self.url
    }
}
