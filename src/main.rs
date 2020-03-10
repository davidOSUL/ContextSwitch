mod curr_time_fetcher;
mod parser;
mod runner;
mod scheduler;
mod website;
mod website_blocker;
use app_dirs::*;

use crate::website_blocker::HostBlocker;
use std::{env, thread};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::error::Error;
use std::time::Duration;

const APP_INFO: AppInfo = AppInfo {
    name: "ContextSwitch",
    author: "CrustyRustys",
};

struct AppData {
    old_host_file: PathBuf,
    old_input_file: PathBuf,
    host_path : PathBuf,
}

static HOST_PATH : &str  = "/etc/hosts";
//static HOST_PATH : &str  = "fakehosts";

impl AppData {
    fn new() -> Result<Self, Box<dyn Error>> {
        let mut old_host_file =
            app_dir(AppDataType::UserData, &APP_INFO, ".old_host")?;
        old_host_file.push(".oldhost");
        let mut old_input_file =
            app_dir(AppDataType::UserData, &APP_INFO, ".old_input")?;
        old_input_file.push(".oldinput");
        Ok(AppData {
            old_host_file,
            old_input_file,
            host_path : PathBuf::from_str(HOST_PATH)?
        })
    }

    fn clear_cache(&self) {
        std::fs::remove_file(&self.old_input_file);
        std::fs::remove_file(&self.old_host_file);
    }
}
type RunnerType = runner::Runner<website_blocker::HostBlocker, curr_time_fetcher::SystemTime>;

fn get_runner_helper(app_data: &AppData, fail_on_old_hosts_exists: bool) -> Result<RunnerType, Box<dyn Error>> {
    let old_input_file =
        File::open(&app_data.old_input_file)?;
    //let old_input_file = File::open(PathBuf::from_str("test.yaml").unwrap()).unwrap();
    let blocks = parser::parse_from_file(old_input_file)?;
    let sched = scheduler::Scheduler::new(blocks)?;
    let blocker =
        website_blocker::HostBlocker::new(fail_on_old_hosts_exists, app_data.host_path.clone(), app_data.old_host_file.clone())
            ?;
    let time_fetcher = curr_time_fetcher::SystemTime::new();
    Ok(runner::Runner::new(sched, blocker, time_fetcher))
}

fn main_runner() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    //let args = vec!["test.yaml".to_owned()];
    let app_data = AppData::new()?;

    //comment out  these line when NOT testing
    //    std::fs::remove_file(&app_data.old_input_file);
    //    std::fs::remove_file(&app_data.old_host_file);

    let fail_on_old_exists = match (args.is_empty(), app_data.old_input_file.exists()) {
        (true, false) => {
            panic!("No session in progress. Start a new context switch session with a file")
        }
        (false, true) => panic!("Existing session in progress -- cannot start a new session"),
        (false, false) => {
            let inputPath = PathBuf::from_str(args[0].as_str())?;
            assert!(inputPath.exists(), "input path doesn't exist");
            std::fs::copy(inputPath, &app_data.old_input_file)?;
            true //case of new session, if an old hosts copy exists, it implies we messed up somehow
        }
        (true, true) => false, //case of resume, don't fail if hosts copy exists
    };

    let runner = get_runner_helper(&app_data, fail_on_old_exists)?;

    runner.start_or_resume()?;
    loop {
        if (runner.poll_finished()) {
            break;
        }
        thread::sleep(Duration::from_millis(2000))
    }

    // if the user quits before this occurs, the old_input_file won't be removed, and so if they try to run it
    // with a new file, it'll panic
    std::fs::remove_file(app_data.old_input_file)?;
    Ok(())
}

fn main() {

    if let Err(e) = main_runner() {
        if let Ok(ad) = AppData::new() {
            ad.clear_cache();
        }
        let s = format!("FATAL ERROR: {:?}", e);
        panic!(s);
    }


}
