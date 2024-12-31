mod tasks;
mod functions;
mod into_task;

use std::sync::{mpsc::{channel, Receiver, Sender}, OnceLock};

pub use tasks::*;

static GLOBAL_SCHEDULER: OnceLock<Scheduler> = OnceLock::new();

pub struct Scheduler {
    /// Tasks that are ran whenever possible
    persistent_tasks: OnceLock<Vec<StoredTask>>,
    /// Tasks needed to be ran only once
    task_sender: Sender<StoredTask>
}

impl Scheduler {
    pub(crate) fn initialize() -> (&'static Scheduler, Receiver<StoredTask>) {
        let (task_sender, task_receiver) = channel();
        let _ = GLOBAL_SCHEDULER.set(
            Scheduler {
                persistent_tasks: OnceLock::new(),
                task_sender,
            }
        );
        (Scheduler::get(), task_receiver)
    }

    pub fn get() -> &'static Scheduler {
        GLOBAL_SCHEDULER.get().unwrap()
    }
}