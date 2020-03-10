use serde::{Deserialize};
use std::fs;
use crate::scheduler::Block;
use chrono::{DateTime, NaiveDateTime, Utc};
use std::error::Error;
use crate::website::Website;

#[derive(Deserialize)]
struct MemoryBlocks {
    blocks: Vec<MemoryBlock>
}

#[derive(Deserialize)]
struct MemoryBlock {
    start_time: String,
    end_time: String,
    dates: Vec<String>,
    websites: Vec<String>
}

pub fn parse_from_file(file_name: &str) -> Result<Vec<Block>, Box<dyn Error>>{
    let file_contents = fs::read_to_string(file_name)?;
    
    let mem_blocks : MemoryBlocks = serde_yaml::from_str(&file_contents)?;

    let mut blocks: Vec<Block> = vec![];

    for block in mem_blocks.blocks {
        for date in block.dates.iter() {

            let sites: Vec<Website> = block.websites.iter().map(|website| Website::from_path(website)).collect::<Result<Vec<Website>, _>>()?;
            
            let start_naive_date_time = NaiveDateTime::parse_from_str(format!("{} {}", date, block.start_time).as_str(), "%Y-%m-%d %H:%M:%S")?;
            let end_naive_date_time = NaiveDateTime::parse_from_str(format!("{} {}", date, block.end_time).as_str(), "%Y-%m-%d %H:%M:%S")?;
            let start_date_time = DateTime::<Utc>::from_utc(start_naive_date_time, Utc);
            let end_date_time = DateTime::<Utc>::from_utc(end_naive_date_time, Utc);
            blocks.push(Block::from_blacklist(sites, start_date_time, end_date_time)?);
        }
    }

    Ok(blocks)
}