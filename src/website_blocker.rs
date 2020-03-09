use std::fs::OpenOptions;
use std::fs::File;
use std::fs::copy;
use std::io::prelude::*;
use std::path::PathBuf;
use std::collections::HashSet;
use crate::errors::BlockerError;

use crate::website::*;

struct HostBlocker {
    hosts_path: PathBuf,
    redirect: &'static str,
    blocked_sites: HashSet<Website>
}

trait WebsiteBlocker {
    
    fn block(&mut self, w: &[Website]) -> Result<(), BlockerError>;
    
    fn unblock(&mut self, w: &[Website]) -> Result<(), BlockerError>;

    fn clear(&mut self) -> Result<(), BlockerError>;
}

impl WebsiteBlocker for HostBlocker {
    
    fn block(&mut self, w: &[Website]) -> Result<(), BlockerError> {
        //let mut file_contents = String::new();
        
        //hosts_file.read_to_string(&mut file_contents).map_err(|_e| BlockerError::FailedToDeserialize)?;
        
        //let file_words: Vec<&str> = file_contents.split_whitespace().collect();
        for site in w.iter() {
            if !self.blocked_sites.contains(site) {
                self.blocked_sites.insert(site.to_owned());
            } else {
                return Err(BlockerError::SiteAlreadyBlocked);
            }
        }
        
        self.sync_hosts()?;
        Ok(())
    }
    
    fn unblock(&mut self, w: &[Website]) -> Result<(), BlockerError> {
        for site in w.iter() {
            if self.blocked_sites.contains(site) {
                self.blocked_sites.remove(site);
            } else {
                return Err(BlockerError::SiteAlreadyBlocked);
            }
        }
        
        self.sync_hosts()?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), BlockerError> {
        self.reset_hosts()?;
        let hosts2_path = self.hosts_path.join("2");
        std::fs::remove_file(hosts2_path).map_err(|_e| BlockerError::FailedToSerialize)?;
        self.blocked_sites.clear();
        Ok(())
    }
}

impl HostBlocker {

    fn new() ->  Result<(Self), BlockerError> { // todo parse hosts file and see what is already blocked or unblock everything

        let new_hostblocker = HostBlocker { 
            hosts_path: PathBuf::from("/etc/hosts"),
            redirect: "127.0.0.1",
            blocked_sites: HashSet::new()  
        };

        new_hostblocker.save_hosts()?; // todo: handle case where hosts2 exists
        Ok(new_hostblocker)
    }

    // resets hosts back to its original state and rewrites all the websites that need to be blocked
    fn sync_hosts(&self) -> Result<(), BlockerError> {
        self.reset_hosts()?;

        let mut hosts_file = OpenOptions::new()
                                            .read(true)
                                            .append(true)
                                            .open(&self.hosts_path)
                                            .map_err(|_e| BlockerError::FailedToDeserialize)?;

        for site in self.blocked_sites.iter() {
            hosts_file.write(format!("\n{} {}", self.redirect, site.get_url().as_str()).as_bytes())
            .map_err(|_e| BlockerError::FailedToSerialize)?;            
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

        copy(hosts2_path, &self.hosts_path).map_err(|_e| BlockerError::FailedToDeserialize)?;
        Ok(())
    }

    fn save_hosts(&self) -> Result<(), BlockerError> {
        if !self.hosts_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }
        
        let hosts2_path = self.hosts_path.join("2");
        
        //todo: if hosts2 exists at this point we have hit the edge case where the program quit early. Handle this somehow.
        File::create(&hosts2_path).map_err(|_e| BlockerError::FailedToDeserialize)?; 
        
        copy(&self.hosts_path, &hosts2_path).map_err(|_e| BlockerError::FailedToDeserialize)?;
        Ok(())
    }
}