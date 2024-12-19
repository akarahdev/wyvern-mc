use async_executor::LocalExecutor;
use voxidian_protocol::packet::PacketBuf;
use crate::Server;

impl Server {
    pub fn start(self) {
        loop {
            for connection in &self.connections {
                // do something
            }
        }
    }
}