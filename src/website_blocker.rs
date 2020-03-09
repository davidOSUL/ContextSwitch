use std::fs::OpenOptions;
use std::fs::File;
use std::fs::copy;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use std::collections::HashSet;

use crate::website::*;

enum BlockerError { // todo convert this into an actual error type
    SiteAlreadyBlocked,
    SiteAlreadyUnblocked,
    FailedToSerialize,
    FailedToDeserialize
}

struct HostBlocker {
    hosts_path: PathBuf,
    redirect: &'static str,
    blocked_sites: HashSet<Website>
}

trait WebsiteBlocker {
    fn new() -> Result<(Self), Box<dyn Error>> where Self: Sized;
    
    fn block(&self, w: Website) -> Result<(), BlockerError>;
    
    fn unblock(&self, w: Website) -> Result<(), BlockerError>;

    fn clear(&self) -> Result<(), Box<dyn Error>>;
}

impl WebsiteBlocker for HostBlocker {
    fn new() ->  Result<(Self), Box<dyn Error>> { // todo parse hosts file and see what is already blocked or unblock everything

        let new_hostblocker = HostBlocker { 
            hosts_path: PathBuf::from("/etc/hosts"),
            redirect: "127.0.0.1",
            blocked_sites: HashSet::new()  
        };

        new_hostblocker.save_hosts()?; // todo: handle case where hosts2 exists
        Ok(new_hostblocker)
    }
    
    fn block(&self, w: Website) -> Result<(), BlockerError> {
        //let mut file_contents = String::new();
        
        //hosts_file.read_to_string(&mut file_contents).map_err(|_e| Err(BlockerError::FailedToDeserialize))?;
        
        //let file_words: Vec<&str> = file_contents.split_whitespace().collect();

        if !self.blocked_sites.contains(&w) {
            self.blocked_sites.insert(w);
            self.sync_hosts()?;
            Ok(())
        } else {
            Err(BlockerError::SiteAlreadyBlocked)
        }

    }
    
    fn unblock(&self, w: Website) -> Result<(), BlockerError> {


        if self.blocked_sites.contains(&w) {
            self.blocked_sites.remove(&w);
            self.sync_hosts()?;
            Ok(())
        } else {
            Err(BlockerError::SiteAlreadyUnblocked)
        }
    }

    fn clear(&self) -> Result<(), BlockerError> {
        self.reset_hosts();
        let hosts2_path = self.hosts_path.join("2");
        std::fs::remove_file(hosts2_path).map_err(|_e| Err(BlockerError::FailedToSerialize))?;
        self.blocked_sites.clear();
        Ok(())
    }
}

impl HostBlocker {

    // resets hosts back to its original state and rewrites all the websites that need to be blocked
    fn sync_hosts(&self) -> Result<(), BlockerError> {
        self.reset_hosts();

        let mut hosts_file = OpenOptions::new()
                                            .read(true)
                                            .append(true)
                                            .open(self.hosts_path)
                                            .map_err(|_e| Err(BlockerError::FailedToDeserialize))?;

        for site in self.blocked_sites.iter() {
            hosts_file.write(format!("\n{} {}", self.redirect, site.get_url().as_str()).as_bytes())
            .map_err(|_e| Err(BlockerError::FailedToSerialize))?;            
        }
        Ok(())
    }

    // resets hosts back to its original state
    fn reset_hosts(&self) -> Result<(), BlockerError> {
        if !self.hosts_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }

        let hosts2_path = self.hosts_path.join("2");

        if !hosts2_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }

        copy(hosts2_path, self.hosts_path).map_err(|_e| Err(BlockerError::FailedToDeserialize))?;
    }

    fn save_hosts(&self) -> Result<(), BlockerError> {
        if !self.hosts_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }
        
        let hosts2_path = self.hosts_path.join("2");
        
        //todo: if hosts2 exists at this point we have hit the edge case where the program quit early. Handle this somehow.
        let mut hosts2_file = File::create(hosts2_path).map_err(|_e| Err(BlockerError::FailedToDeserialize))?; 
        
        copy(self.hosts_path, hosts2_path).map_err(|_e| Err(BlockerError::FailedToDeserialize))?;
                
    }
}