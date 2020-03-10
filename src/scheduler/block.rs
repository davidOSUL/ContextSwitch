use crate::website::Website;
use chrono::{DateTime, TimeZone, NaiveDateTime};
use std::collections::HashSet;
use std::iter::FromIterator;
use serde::{Serialize, Deserialize};
use serde_yaml;

pub enum BlockError {
    StartAfterEnd
}

pub type Timestamp = i64;

#[derive(Serialize, Deserialize)]
pub enum BlockList {
    Whitelist(HashSet<Website>),
    Blacklist(HashSet<Website>)
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    list : BlockList,
    time_start : NaiveDateTime,
    time_end : NaiveDateTime
}


impl BlockList {
    /// returns true if the two lists don't contradict each other
    pub fn coexist(&self, other_list : &BlockList) -> bool {
        // this is not entirely accurate, obviously. Just doing the simple "rule" for now of allowing
        // two blacklists but no other combinations
        match (&self, other_list) {
            (BlockList::Blacklist(_), BlockList::Blacklist(_))  => true,
            _ => false
        }
    }

    pub fn should_block(&self, website : &Website) -> bool {
        match &self {
            BlockList::Blacklist(l) => l.contains(website),
            BlockList::Whitelist(l) => !l.contains(website)
        }
    }
}


impl Block {
    pub fn from_blacklist<Tz : TimeZone, Tz2: TimeZone>(blacklist : &[Website], time_start: DateTime<Tz>, time_end: DateTime<Tz2>) -> Result<Self, BlockError>  {
        if time_start > time_end {
            return Err(BlockError::StartAfterEnd);
        }
        Ok(Block {
            list : BlockList::Blacklist(HashSet::from_iter(blacklist.to_vec().into_iter())),
            time_start: time_start.naive_utc(),
            time_end : time_end.naive_utc()
        })
    }


    pub fn contains_time<Tz2 : TimeZone> (&self, time : DateTime<Tz2>) -> bool {
        time.naive_utc() >= self.time_start && time.naive_utc() <= self.time_end
    }

    pub fn list(&self) -> &BlockList {
        &self.list
    }

    pub fn start_timestamp(&self) -> Timestamp {
        self.time_start.timestamp()
    }

    pub fn end_timestamp(&self) -> Timestamp {
        self.time_end.timestamp()
    }

//    pub fn getRange(&self) -> RangeInclusive<Timestamp> {
//        RangeInclusive::new(self.start_timestamp(), self.end_timestamp())
//    }

    pub fn get_range(&self) -> std::ops::Range<Timestamp> {
        std::ops::Range {
            start: self.start_timestamp(),
            end: self.end_timestamp()+1
        }
    }

    pub fn serialize<T : Write>(&self, writer : &mut T) -> Result<(), Box<dyn Error>> {
        let yaml = serde_yaml::to_string(&self)?;
        writer.write_all(yaml.to_bytes())?;
        Ok(())
    }

    pub fn from_serialized<T: Read>(read : t) -> Result<Block, Error> {
        serde_yaml::from_reader(t)
    }
}


