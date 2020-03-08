extern crate chrono;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // MacOS hosts path
    let hosts_path = Path::new("/etc/hosts");

    if !hosts_path.exists() {
        println!("uh oh spagetti-o's");
    }

    // localhost's IP
    let redirect = "127.0.0.1";
    let website_list = [
        "www.facebook.com",
        "facebook.com",
        "dub119.mail.live.com",
        "www.dub119.mail.live.com",
        "www.gmail.com",
        "gmail.com",
        "reddit.com",
        "www.reddit.com",
    ];

    let mut hosts_file = OpenOptions::new().read(true).append(true).open(hosts_path).unwrap(); //todo handle unwrap

    let mut file_contents = String::new();
    
    hosts_file.read_to_string(&mut file_contents).unwrap(); //todo handle unwrap
    
    let file_words: Vec<&str> = file_contents.split_whitespace().collect();

    website_list
        .iter()
        .filter(|website| !file_words.contains(*website))
        .for_each(|website| {
            hosts_file
                .write(format!("\n{} {}", redirect, website).as_bytes())
                .unwrap(); //todo handle unwrap
        }); 
}

