#![allow(dead_code)]

use crate::scheduler::Block;
use crate::website::Website;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Deserialize)]
struct MemoryBlocks {
    blocks: HashMap<String, MemoryBlock>,
}

#[derive(Deserialize)]
struct MemoryBlock {
    start_time: String,
    end_time: String,
    dates: Vec<String>,
    websites: Vec<String>,
}

pub fn parse_from_file(mut file: File) -> Result<Vec<Block>, Box<dyn Error>> {
    let mem_blocks: MemoryBlocks = serde_yaml::from_reader(file)?;

    let mut blocks: Vec<Block> = vec![];

    for block in mem_blocks.blocks.values() {
        for date in &block.dates {
            let sites: Vec<Website> = block
                .websites
                .iter()
                .map(|website| Website::from_path(website))
                .collect::<Result<Vec<Website>, _>>()?;

            let start_local_date_time = Local.datetime_from_str(
                format!("{} {}", date, block.start_time).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )?;
            let end_local_date_time = Local.datetime_from_str(
                format!("{} {}", date, block.end_time).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )?;

            blocks.push(Block::from_blacklist(
                sites,
                start_local_date_time,
                end_local_date_time,
            )?);
        }
    }

    Ok(blocks)
}
