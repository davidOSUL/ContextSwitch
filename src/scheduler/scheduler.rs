use crate::scheduler::Block;
use crate::scheduler::block::Timestamp;
use chrono::{TimeZone, DateTime, NaiveDateTime};
use intervaltree::{IntervalTree, Element};
use std::collections::HashMap;
use crate::scheduler::block::BlockList;
use std::iter::FromIterator;
use std::time::SystemTime;

pub enum SchedulerError {
    Contradiction
}

type Key = Timestamp;
type Value = Vec<Block>;
type RangeType = core::ops::Range<Key>;
type TreeType = IntervalTree<Key, Value>;
type ElemType = Element<Key, Value>;

pub struct Scheduler {
    blocks : TreeType, // first element in key is timestamp, second is count (since multiple blockcs could have same timestamp
}

impl Scheduler {
    pub fn new(&mut self, blocks : Vec<Block>) -> Result<Self, SchedulerError> {

        // acount for blocks having duplicate ranges
        let mut elements = HashMap::<RangeType, ElemType>::new();
        for b in blocks {
            let e = elements.entry(b.get_range()).or_insert(ElemType {
                range: b.get_range(),
                value: Vec::new()
            });
            e.value.push(b);
        }

        //construct interval tree from elements
        let iter = elements.into_iter().map(|x| x.1);
        let interval_tree = IntervalTree::from_iter(iter);


        // for every block in the tree, check it against all blocks that lie within it's interval
        // if there is a contradiction, return an error
        for e in &interval_tree {
            for block in &e.value {
                for overlaps in interval_tree.query(e.range.clone()) {
                    for overlapped_block in &overlaps.value {
                        if !(overlapped_block.list().coexist(block.list())) {
                            return Err(SchedulerError::Contradiction);
                        }
                    }
                }
            }

        }

        Ok(Scheduler {
            blocks: interval_tree
        })

    }

    pub fn get_block_list<Tz: TimeZone>(&self, curr_time : DateTime<Tz>) -> Vec<&BlockList> {
        self.blocks.query_point(curr_time.timestamp()).flat_map(|e| &e.value)
            .map(|b| b.list()).collect()
    }

}