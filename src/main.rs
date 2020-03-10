mod runner;
mod website_blocker;
mod website;
mod scheduler;
mod curr_time_fetcher;
mod errors;
mod parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // parse the first argument (a text file)
    let blocks = parser::parse_from_file(&args[0]); // David: do what you want with this vector of blocks
}
