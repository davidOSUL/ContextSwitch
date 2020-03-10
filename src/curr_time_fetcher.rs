use chrono::{DateTime, Utc};

pub struct SystemTime;
pub trait CurrTimeFetcher {
    fn now(&self) -> DateTime<Utc>;
}

impl SystemTime {
    pub fn new() -> Self {Self}
}

impl CurrTimeFetcher for SystemTime {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
