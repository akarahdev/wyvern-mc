use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}};

use voxidian_protocol::{packet::s2c::play::{BlockUpdateS2CPlayPacket, WorldChunkWithLightS2CPlayPacket}, registry::RegEntry, value::{BlockPos, BlockState as ProtocolBlockState, ChunkSectionData, Nbt, NbtCompound}};

use crate::{values::{BlockPosition, ChunkPosition, ChunkSectionPosition, Key, Location}, Server};

use super::{BlockState, ChunkSection, DimensionData};

#[derive(Clone)]
pub struct Dimension {
    inner: Arc<Mutex<DimensionData>>
}

impl Dimension {
    pub fn new(name: Key<Dimension>) -> Dimension {
        Dimension {
            inner: Arc::new(Mutex::new(DimensionData { 
                name, 
                chunk_sections: HashMap::new(),
                min_y: 0,
                max_y: 16
            }))
        }
    }

    pub fn name(&self) -> Key<Dimension> {
        self.inner.lock().unwrap().name.clone()
    }

    pub(crate) fn get_chunk_as_packets(&self, cp: ChunkPosition) -> WorldChunkWithLightS2CPlayPacket {
        let mut inner = self.inner.lock().unwrap();

        WorldChunkWithLightS2CPlayPacket {
            chunk_x: cp.x as i32,
            chunk_z: cp.z as i32,
            heightmaps: Nbt { name: "".to_string(), root: NbtCompound::new() },
            data: ChunkSectionData {
                sections: {
                    let mut sections = Vec::new();
                    for chunk_y in (inner.min_y..inner.max_y).step_by(16) {
                        let cp = ChunkSectionPosition::new(cp.x, chunk_y, cp.z);
                        let chunk = match inner.chunk_sections.get_mut(&cp) {
                            Some(chunk) => chunk,
                            None => {
                                inner.chunk_sections.insert(cp, ChunkSection::default());
                                inner.chunk_sections.get_mut(&cp).unwrap()
                            },
                        };
                        sections.push(chunk.to_packet());
                    }
                    sections
                },
            },
            block_entities: vec![].into(),
            sky_light_mask: vec![].into(),
            block_light_mask: vec![].into(),
            empty_sky_light_mask: vec![].into(),
            empty_block_light_mask: vec![].into(),
            sky_light_array: vec![].into(),
            block_light_array: vec![].into(),
        }
    }

    pub fn set_block(&mut self, location: BlockPosition, block: BlockState) {
        let mut inner = self.inner.lock().unwrap();

        let chunk = match inner.chunk_sections.get_mut(&location.to_chunk_section_pos()) {
            Some(chunk) => chunk,
            None => {
                inner.chunk_sections.insert(
                    location.to_chunk_section_pos(), 
                    ChunkSection::default()
                );
                inner.chunk_sections.get_mut(&location.to_chunk_section_pos()).unwrap()
            },
        };

        chunk.set_block_at(location.x as usize % 15, location.y as usize % 15, location.z as usize % 15, block);
    }

    pub fn get_block(&mut self, location: BlockPosition) -> BlockState {
        let mut inner = self.inner.lock().unwrap();

        let chunk = match inner.chunk_sections.get_mut(&location.to_chunk_section_pos()) {
            Some(chunk) => chunk,
            None => {
                inner.chunk_sections.insert(
                    location.to_chunk_section_pos(), 
                    ChunkSection::default()
                );
                inner.chunk_sections.get_mut(&location.to_chunk_section_pos()).unwrap()
            },
        };

        BlockState::from_protocol(
            voxidian_protocol::value::BlockState::from_id(
                chunk.block_at(location.x as usize % 15, location.y as usize % 15, location.z as usize % 15).id() as i32
            ).unwrap()
        )
    }
}