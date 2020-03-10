use crate::scheduler::block::BlockList;
use crate::scheduler::block::Timestamp;
use crate::scheduler::Block;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use intervaltree::{Element, IntervalTree};
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Input has contradiction in blocked time. Ensure contexts make sense and try again")]
    Contradiction,
}

type Key = Timestamp;
pub type Value = (usize, Vec<Block>);
type RangeType = core::ops::Range<Key>;
type TreeType = IntervalTree<Key, Value>;
type ElemType = Element<Key, Value>;

pub struct Scheduler {
    max_end: Timestamp,
    blocks: TreeType, // first element in key is timestamp, second is count (since multiple blockcs could have same timestamp
}

impl Scheduler {
    pub fn new(blocks: Vec<Block>) -> Result<Self, SchedulerError> {
        let mut id: usize = 0;
        let mut max_end: Timestamp = 0;
        // acount for blocks having duplicate ranges
        let mut elements = HashMap::<RangeType, ElemType>::new();
        for b in blocks {
            max_end = max(max_end, b.end_timestamp());
            let e = elements.entry(b.get_range()).or_insert_with(|| {
                id += 1;
                ElemType {
                    range: b.get_range(),
                    value: (id, Vec::new()),
                }
            });
            e.value.1.push(b);
        }

        //construct interval tree from elements
        let iter = elements.into_iter().map(|x| x.1);
        let interval_tree = IntervalTree::from_iter(iter);

        // for every block in the tree, check it against all blocks that lie within it's interval
        // if there is a contradiction, return an error
        for e in &interval_tree {
            for block in &e.value.1 {
                for overlaps in interval_tree.query(e.range.clone()) {
                    for overlapped_block in &overlaps.value.1 {
                        if !(overlapped_block.list().coexist(block.list())) {
                            return Err(SchedulerError::Contradiction);
                        }
                    }
                }
            }
        }

        Ok(Scheduler {
            max_end,
            blocks: interval_tree,
        })
    }

    pub fn is_past_end<Tz: TimeZone>(&self, curr_time: &DateTime<Tz>) -> bool {
        let x = curr_time.timestamp();
        return curr_time.timestamp() > self.max_end;
    }

    pub fn get_block_ids<Tz: TimeZone>(&self, curr_time: &DateTime<Tz>) -> HashSet<usize> {
        self.blocks
            .query_point(curr_time.timestamp())
            .map(|e| e.value.0)
            .collect()
    }

    pub fn get_block_list<Tz: TimeZone>(&self, curr_time: &DateTime<Tz>) -> Vec<&BlockList> {
        self.blocks
            .query_point(curr_time.timestamp())
            .flat_map(|e| &e.value.1)
            .map(|b| b.list())
            .collect()
    }
}
