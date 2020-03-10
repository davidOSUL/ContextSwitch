use crate::curr_time_fetcher::CurrTimeFetcher;
use crate::scheduler::Scheduler;
use crate::website_blocker::{WebsiteBlocker, BlockerError};
use std::collections::HashSet;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
use thiserror::Error;
use std::marker::PhantomData;
use std::marker::Send;

pub struct Runner<T: WebsiteBlocker + Send + 'static, V: CurrTimeFetcher + Send + 'static> {
    blocker_type : std::marker::PhantomData<T>,
    fetcher_type : std::marker::PhantomData<V>,
    sender: Sender<Msg>,
}

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Failed to start or resume Runner")]
    FailedToStartOrResume,
    #[error("Failed to pause runner Runner")]
    FailedToPause,
}

enum State {
    Running(HashSet<usize>),
    Pausing,
}

struct Worker<T: WebsiteBlocker + Send + 'static, V: CurrTimeFetcher + Send + 'static> {
    time_fetcher : V,
    sched: Scheduler,
    blocker: T,
    state: State,
}

enum Msg {
    Pause,
    Resume,
    NoOpPoll, //used to see if worker is still active
}

impl<T: WebsiteBlocker + Send + 'static, V: CurrTimeFetcher + Send + 'static> Worker<T, V> {
    fn new(sched: Scheduler, blocker: T, time_fetcher: V, state: State) -> Self {
        Worker {
            time_fetcher,
            sched,
            blocker,
            state,
        }
    }

    fn spawn_worker_thread(mut worker: Worker<T, V>) -> Sender<Msg> {
        let (tx, rx) = channel::<Msg>();

        thread::spawn(move || {
            loop {
                if worker.check_work_is_done().unwrap() {
                    drop(rx); //communicate that we are done reciecing messages cause there's nothing left to do
                    break;
                }
                match rx.try_recv() {
                    Ok(msg) => worker.handle_msg(msg).unwrap(),
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => (),
                }
                worker.update_blocks().unwrap();
                thread::sleep(Duration::from_millis(100))
            }
        });
        tx
    }

    fn check_work_is_done(&mut self) -> Result<bool, BlockerError> {
        if self.sched.is_past_end(&self.time_fetcher.now()) {
            self.blocker.clear()?;
            return Ok(true);
        }
        Ok(false)
    }

    fn handle_msg(&mut self, m: Msg) -> Result<(), BlockerError>{
        match (&self.state, m) {
            (State::Running(_), Msg::Pause) => {
                self.blocker.clear()?;
                self.state = State::Pausing
            }
            (State::Pausing, Msg::Resume) => {
                self.state = State::Running(HashSet::new());
            }
            _ => (),
        };
        Ok(())
    }

    fn update_blocks(&mut self) -> Result<(), BlockerError> {
        match &self.state {
            State::Pausing => return Ok(()),
            State::Running(hs) => {
                let now = self.time_fetcher.now();
                let ids = self.sched.get_block_ids(&now);
                if hs.eq(&ids) {
                    return Ok(());
                }
                let values = self.sched.get_block_list(&now).iter().flat_map(|bl| bl.get_list()).cloned()
                    .collect::<Vec<_>>();
                self.blocker.set_block_list(values)?;
                self.state = State::Running(ids);

            }
        };
        Ok(())
    }
}

impl<T: WebsiteBlocker + Send + 'static, V: CurrTimeFetcher + Send + 'static> Runner<T, V> {
    pub fn new(sched: Scheduler, blocker: T, time_fetcher: V) -> Self {
        let w = Worker::new(sched, blocker, time_fetcher,State::Pausing);
        let sender = Worker::spawn_worker_thread(w);
        Runner { blocker_type: PhantomData, fetcher_type: PhantomData, sender }
    }

    pub fn poll_finished(&self) -> bool {
        self.sender.send(Msg::NoOpPoll).is_err()
    }

    pub fn start_or_resume(&self) -> Result<(), RunnerError> {
        self.sender
            .send(Msg::Resume)
            .map_err(|e| RunnerError::FailedToStartOrResume)
    }

    pub fn pause(&self) -> Result<(), RunnerError> {
        self.sender
            .send(Msg::Pause)
            .map_err(|e| RunnerError::FailedToPause)
    }
}
