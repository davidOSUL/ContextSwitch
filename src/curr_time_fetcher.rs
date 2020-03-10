use chrono::{DateTime, Utc};

pub struct SystemTime;
pub trait CurrTimeFetcher {
    fn now() -> DateTime<UTC>;
}

impl CurrTimeFetcher for SystemTime {
    fn now() -> DateTime<UTC> {
        Utc::now()
    }
}