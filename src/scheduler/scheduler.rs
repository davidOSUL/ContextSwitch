use crate::scheduler::Block;

enum SchedulerError {

}

pub struct Scheduler {

}

impl Scheduler {
    pub fn add_block<Tz : Timezone>(&self, b : &Block<Tz>) -> Result<(), SchedulerError>{

    }

}