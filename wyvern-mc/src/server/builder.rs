use std::net::ToSocketAddrs;

use crate::{plugin::Plugin, scheduler::{into_task::IntoTask, parameters::TaskParameter, Scheduler, StoredTask}};

use super::{ConfigEvent, HandshakeEvent, LoginEvent, PlayEvent, Server, StatusEvent};

pub struct ServerBuilder {
    pub(crate) server: Server,
    pub(crate) persistent_tasks: Vec<StoredTask>
}

impl ServerBuilder {
    pub fn add_system<I: TaskParameter, S: IntoTask<I>>(&mut self, task: S) -> &Self {
        self.persistent_tasks.push(Box::new(task.into_task()));
        self
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.load(self);
        self
    }

    pub fn low_level<F: FnOnce(UnsafeServerBuilder)>(
        &self,
        f: F,
    ) {
        let handle = UnsafeServerBuilder {
            server: self.server.clone()
        };
        f(handle);
    }

    pub fn start<S: ToSocketAddrs>(self, addr: S) {
        let (_, receiver) = Scheduler::initialize();
        self.server.inner.lock().unwrap().task_receiver.set(receiver).unwrap();
        let mut rf = Scheduler::get().persistent_tasks.lock().unwrap();
        *rf = self.persistent_tasks;
        drop(rf);
        self.server.start(addr);
    }
}

pub struct UnsafeServerBuilder {
    pub(crate) server: Server
}

impl UnsafeServerBuilder {
    pub fn handshake_event(&self, event: HandshakeEvent) {
        self.server.inner.lock().unwrap().handshake_events.push(event);
    }

    pub fn status_event(&self, event: StatusEvent) {
        self.server.inner.lock().unwrap().status_events.push(event);
    }

    pub fn login_event(&self, event: LoginEvent) {
        self.server.inner.lock().unwrap().login_events.push(event);
    }

    pub fn configuration_event(&self, event: ConfigEvent) {
        self.server.inner.lock().unwrap().config_events.push(event);
    }

    pub fn play_event(&self, event: PlayEvent) {
        self.server.inner.lock().unwrap().play_events.push(event);
    }
}