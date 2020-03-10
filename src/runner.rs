use crate::scheduler;
use crate::website_blocker;
use crate::scheduler::Scheduler;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread;
use crate::website::Website;
use crate::website_blocker::WebsiteBlocker;
use chrono::DateTime;
use crate::curr_time_fetcher::CurrTimeFetcher;

pub struct Runner<T : WebsiteBlocker> {
    sender : Sender<Msg>
}

enum State {
    Running,
    Pausing
}


struct Worker<T : WebsiteBlocker, V : CurrTimeFetcher> {
    sched : Scheduler,
    blocker : T,
    state : State
}

enum Msg {
    Pause,
    Resume
}

impl<T : WebsiteBlocker, V : CurrTimeFetcher> Worker<T, V> {
    fn spawn_worker(mut worker: Worker<T, V>) -> Sender<Msg> {
        let (tx, rx) = channel::<Msg>();

        thread::spawn(move || {
            loop {
                match rx.try_recv() {
                    Ok(msg) => worker.handle_msg(msg),
                    Err(TryRecvError::Disconnected) => break,
                    Err(TryRecvError::Empty) => ()
                }
                worker.update_blocks();
            }
        });
        tx
    }

    fn handle_msg(&mut self, m : msg)  {
        let old_state = &self.state;
        self.state = match Msg {
            Msg::Pause => State::Pausing,
            Msg::Resume => State::Running,
        };

        if old_state == State::Running && self.state == State::Pausing {
            self.blocker.clear();
        }
    }

    fn update_blocks(&mut self) {
        if self.state == State::Pausing {
            return;
        }
        self.sched.get_block_list(V::now());


    }
}

impl<T : WebsiteBlocker> Runner<T> {

    fn new(sched : Scheduler, blocker: T) -> Self {
        Runner {

        }
        Runner {
            sched,
            blocker : Box::new(blocker)
        }
    }

    fn start(&self) ->

    fn spawn_thread(worker : Worker<T>) -> Sender<Msg> {
        let (tx, rx) = channel::<Msg>();

        thread::spawn(move || {
            loop {
                if let Ok(msg) = rx.try_recv() {
                    match msg {
                        Msg::Pause =>
                        Msg::Resume =>
                    }
                }
            }
        });
        tx
    }

}


/// creates a new runner object. Takes ownership of scheduler
Runner::new(sched : Scheduler, blocker: WebsiteBlocker) -> Self

/// Will block websites as needed at the appropriate time according to the blocks that reside within it.
/// This most likely will be done by spawning a thread to periodically poll
/// owned scheduler for the block list. Will fail if already running.
Runner::start(&self) -> Result<(), Error>

/// It is up to the respective front-end to decide if this should be allowable to users, but this is an exposed method regardless. WIll fail if not running.
Runner::stop() -> Result<(), Error>

/// for stretch goal -- make use of something akin to the observer pattern to allow for notifications
Runner::addObserver(o : &Observer) -> ()