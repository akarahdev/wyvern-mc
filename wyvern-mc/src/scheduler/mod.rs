mod tasks;
mod event;
mod type_map;
pub mod functions;
pub mod into_task;
pub mod parameters;

use std::sync::{mpsc::{channel, Receiver, Sender}, Mutex, OnceLock};

use into_task::IntoTask;
pub use tasks::*;
pub use event::*;
pub use type_map::*;

pub(crate) static GLOBAL_SCHEDULER: OnceLock<Scheduler> = OnceLock::new();

pub struct Scheduler {
    /// Tasks that are ran whenever possible
    pub(crate) persistent_tasks: Mutex<Vec<StoredTask>>,
    /// Tasks needed to be ran only once
    pub(crate) task_sender: Sender<StoredTask>
}

impl Scheduler {
    pub(crate) fn initialize() -> (&'static Scheduler, Receiver<StoredTask>) {
        let (task_sender, task_receiver) = channel();
        let _ = GLOBAL_SCHEDULER.set(
            Scheduler {
                persistent_tasks: Mutex::new(Vec::new()),
                task_sender,
            }
        );
        (Scheduler::get(), task_receiver)
    }

    pub fn get() -> &'static Scheduler {
        GLOBAL_SCHEDULER.get().unwrap()
    }

    pub fn spawn<I, F: IntoTask<I>>(f: F) {
        let _ = Scheduler::get().task_sender.send(Box::new(f.into_task()));
    }

    pub fn run_systems_with_map(data: &TypeMap) {
        let mut tasks = Scheduler::get().persistent_tasks.lock().unwrap();
        for task in tasks.iter_mut() {
            task.run(&data);
        }
    }
}