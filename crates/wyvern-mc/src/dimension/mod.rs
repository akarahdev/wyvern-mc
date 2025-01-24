use std::collections::HashMap;

use chunk::ChunkSection;
use message::DimensionMessage;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use voxidian_protocol::value::DimType;

use crate::{
    server::Server,
    values::{Key, Position},
};

pub mod blocks;
pub mod chunk;
pub(crate) mod dimension;
pub mod message;

pub use dimension::*;

#[allow(dead_code)]
pub struct DimensionData {
    pub(crate) name: Key<DimensionData>,
    pub(crate) chunks: HashMap<Position<i32>, ChunkSection>,
    pub(crate) server: Option<Server>,
    pub(crate) rx: Receiver<DimensionMessage>,
    pub(crate) tx: Sender<DimensionMessage>,
    pub(crate) dim_type: Key<DimType>,
}

impl DimensionData {
    pub(crate) fn new(
        name: Key<DimensionData>,
        server: Server,
        dim_type: Key<DimType>,
    ) -> DimensionData {
        let chan = channel(1024);
        DimensionData {
            name,
            chunks: HashMap::new(),
            server: Some(server),
            rx: chan.1,
            tx: chan.0,
            dim_type,
        }
    }

    pub fn default_chunk(&mut self, pos: &Position<i32>) {
        if !self.chunks.contains_key(pos) {
            self.chunks.insert(*pos, ChunkSection::empty());
        }
    }

    pub async fn handle_messages(mut self) {
        loop {
            if let Some(msg) = self.rx.recv().await {
                match msg {
                    DimensionMessage::GetChunkSection(position, sender) => {
                        let chunk = self.chunks.get(&position);
                        let _ = sender.send(chunk.cloned());
                    }
                    DimensionMessage::GetDimensionType(sender) => {
                        let _ = sender.send(self.dim_type.clone());
                    }
                    DimensionMessage::SetBlockAt(position, block_state) => {
                        let chunk = position.map_into_coords(|x| x / 16);
                        let pos_in_chunk = position.map_into_coords(|x| (x % 16) as usize);

                        if !self.chunks.contains_key(&chunk) {
                            self.chunks.insert(chunk.clone(), ChunkSection::empty());
                        }

                        let chunk = self.chunks.get_mut(&chunk).unwrap();
                        chunk.set_block_at(pos_in_chunk, block_state);
                    }
                    DimensionMessage::GetBlockAt(position, sender) => {
                        let chunk = position.map_into_coords(|x| x / 16);
                        let pos_in_chunk = position.map_into_coords(|x| (x % 16) as usize);

                        if !self.chunks.contains_key(&chunk) {
                            self.chunks.insert(chunk.clone(), ChunkSection::empty());
                        }

                        let chunk = self.chunks.get_mut(&chunk).unwrap();
                        let block_state = chunk.get_block_at(pos_in_chunk);
                        let _ = sender.send(block_state);
                    }
                }
            };
        }
    }
}
