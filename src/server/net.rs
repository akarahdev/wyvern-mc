use std::rc::Rc;
use std::sync::{Arc, Mutex};
use async_executor::LocalExecutor;
use voxidian_protocol::packet::PacketBuf;
use crate::Server;

impl Server {
    pub fn start(self) {
        let executor = Rc::new(LocalExecutor::new().leak());
        let in_task_executor = executor.clone();
        executor.spawn(async move {
            loop {
                for connection in &self.connections {
                    let conn = connection.clone();
                    let sub_task_executor = in_task_executor.clone();
                    in_task_executor.spawn(async move {
                        // TODO: read packet byte(s)
                    }).detach();

                    let conn = connection.clone();
                    let sub_task_executor = in_task_executor.clone();
                    in_task_executor.spawn(async move {
                        // TODO: write packet byte(s)
                    }).detach();
                }
            }
        }).detach();

    }
}