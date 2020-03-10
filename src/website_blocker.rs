use std::collections::HashSet;
use std::fs::copy;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use thiserror::Error;

use crate::website::*;
use std::iter::FromIterator;

#[derive(Debug, Error)]
pub enum BlockerError {
    #[error("Failed to serialize")]
    FailedToSerialize,
    #[error("Failed to deserialize")]
    FailedToDeserialize,
    #[error("Failed on exist")]
    FailOnExist,
}

pub struct HostBlocker {
    hosts_path: PathBuf,
    hosts2_path: PathBuf,
    redirect: &'static str,
    blocked_sites: HashSet<Website>,
}

pub trait WebsiteBlocker {
    fn set_block_list(&mut self, w: Vec<Website>) -> Result<(), BlockerError>;

    fn clear(&mut self) -> Result<(), BlockerError>;
}

impl WebsiteBlocker for HostBlocker {
    fn set_block_list(&mut self, w: Vec<Website>) -> Result<(), BlockerError> {
        self.blocked_sites = HashSet::from_iter(w.into_iter());
        self.sync_hosts()?;
        Ok(())
    }

    fn clear(&mut self) -> Result<(), BlockerError> {
        self.reset_hosts()?;
        std::fs::remove_file(&self.hosts2_path).map_err(|_e| BlockerError::FailedToSerialize)?;
        self.blocked_sites.clear();
        Ok(())
    }
}

impl HostBlocker {
    pub fn new(fail_on_exists: bool, hosts_path : PathBuf, hosts2_path: PathBuf) -> Result<(Self), BlockerError> {
        let new_hostblocker = HostBlocker {
            hosts_path,
            hosts2_path,
            redirect: "127.0.0.1",
            blocked_sites: HashSet::new(),
        };

        if new_hostblocker.hosts2_path.exists() {
            if fail_on_exists {
                return Err(BlockerError::FailOnExist);
            }
            // if hosts2 exists at this point we have hit the edge case where the program quit early. Revert back to the original host file before we begin.
            new_hostblocker.reset_hosts()?;
            std::fs::remove_file(&new_hostblocker.hosts2_path)
                .map_err(|_e| BlockerError::FailedToSerialize)?;
        }

        new_hostblocker.save_hosts()?;
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

        hosts_file
            .write(format!("\n#Start ContextSwitch Block").as_bytes())
            .map_err(|_e| BlockerError::FailedToSerialize)?;

        for site in self.blocked_sites.iter() {
            hosts_file
                .write(format!("\n{} {}", self.redirect, site.get_url_str()).as_bytes())
                .map_err(|_e| BlockerError::FailedToSerialize)?;
        }

        hosts_file
            .write(format!("\n#End ContextSwitch Block\n").as_bytes())
            .map_err(|_e| BlockerError::FailedToSerialize)?;
        Ok(())
    }

    // resets hosts back to its original state
    fn reset_hosts(&self) -> Result<(), BlockerError> {
        if !self.hosts_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }

        if !self.hosts2_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }



        copy(&self.hosts2_path, &self.hosts_path)
            .map_err(|_e| BlockerError::FailedToDeserialize)?;
        Ok(())
    }

    fn save_hosts(&self) -> Result<(), BlockerError> {
        if !self.hosts_path.exists() {
            return Err(BlockerError::FailedToDeserialize);
        }

        File::create(&self.hosts2_path).map_err(|_e| BlockerError::FailedToDeserialize)?;

        copy(&self.hosts_path, &self.hosts2_path)
            .map_err(|_e| BlockerError::FailedToDeserialize)?;
        Ok(())
    }
}
