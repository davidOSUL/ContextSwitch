use crate::website::Website;
use chrono::{DateTime, TimeZone};
use std::collections::HashSet;
enum BlockError {
    StartAfterEnd
}
enum BlockList {
    Whitelist(HashSet<Website>),
    Blacklist(HashSet<Website>)
}
pub struct Block<Tz : TimeZone> {
    list : BlockList,
    time_start : DateTime<Tz>,
    time_end : DateTime<Tz>
}

impl<Tz : TimeZone> Block<Tz> {
    pub fn from_blacklist(blacklist : &[Website], time_start: DateTime<Tz>, time_end: DateTime<Tz>) -> Result<Self, BlockError>  {
        if time_start > time_end {
            return Err(BlockError::StartAfterEnd);
        }
        Block {
            list : BlockList::Blacklist(blacklist.into()),
            time_start,
            time_end
        }.into()
    }
    pub fn from_whitelist(whitelist : &[Website], time_start: DateTime<Tz>, time_end: DateTime<Tz>) -> Result<Self, BlockError>  {
        if time_start > time_end {
            return Err(BlockError::StartAfterEnd);
        }
        Block {
            list : BlockList::Whitelist(whitelist.into()),
            time_start,
            time_end
        }.into()
    }

    pub fn should_block(&self, website : &Website) -> bool {
        match (&self.list) {
            BlockList::Blacklist(l) => l.contains(website)
            BlockList::Whitelist(l) => !l.contains(website)
        }
    }

    pub fn contains_time<Tz2 : Timezone> (&self, time : DateTime<Tz2>) -> bool {
        time >= self.time_start && time <= self.time_end
    }

    pub fn list(&self) -> &BlockList {
        &self.list
    }
}