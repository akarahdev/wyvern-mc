use std::{collections::HashMap, hash::{Hash, Hasher}, sync::{Arc, Mutex}};

use nohash_hasher::BuildNoHashHasher;
use voxidian_protocol::{packet::s2c::play::LevelChunkWithLightS2CPlayPacket, value::{ChunkSectionData, Nbt, NbtCompound}};

use crate::{values::{BlockPosition, ChunkPosition, ChunkSectionPosition, Key}, Server};

use super::{BlockState, ChunkSection, DimensionData};

#[derive(Clone)]
pub struct Dimension {
    inner: Arc<Mutex<DimensionData>>
}

impl Dimension {
    pub fn new(name: Key<Dimension>) -> Dimension {
        
        let dim = Dimension {
            inner: Arc::new(Mutex::new(DimensionData { 
                name: name.clone(), 
                chunk_sections: HashMap::with_hasher(BuildNoHashHasher::new()),
                min_y: 0,
                max_y: 16
            }))
        };
        Server::get().inner.lock().unwrap().dimensions.insert(name.clone(), dim.clone());
        dim
    }

    pub fn name(&self) -> Key<Dimension> {
        self.inner.lock().unwrap().name.clone()
    }

    pub(crate) fn get_chunk_as_packets(&self, cp: ChunkPosition) -> LevelChunkWithLightS2CPlayPacket {
        let mut inner = self.inner.lock().unwrap();

        LevelChunkWithLightS2CPlayPacket {
            chunk_x: cp.x,
            chunk_z: cp.z,
            heightmaps: Nbt { name: "".to_string(), root: NbtCompound::new() },
            data: ChunkSectionData {
                sections: {
                    let mut sections = Vec::new();
                    for chunk_y in (inner.min_y..inner.max_y).step_by(16) {
                        let cp = ChunkSectionPosition::new(cp.x, chunk_y, cp.z);
                        let nh = cp.map_numeric_hash();
                        let chunk = match inner.chunk_sections.get_mut(&nh) {
                            Some(chunk) => chunk,
                            None => {
                                inner.chunk_sections.insert(nh, ChunkSection::default());
                                inner.chunk_sections.get_mut(&nh).unwrap()
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

        let chunk = match inner.chunk_sections.get_mut(&location.to_chunk_section_pos().map_numeric_hash()) {
            Some(chunk) => chunk,
            None => {
                inner.chunk_sections.insert(
                    location.to_chunk_section_pos().map_numeric_hash(), 
                    ChunkSection::default()
                );
                inner.chunk_sections.get_mut(&location.to_chunk_section_pos().map_numeric_hash()).unwrap()
            },
        };

        chunk.set_block_at(location.x as usize % 15, location.y as usize % 15, location.z as usize % 15, block);
    }

    pub fn get_block(&mut self, location: BlockPosition) -> BlockState {
        let mut inner = self.inner.lock().unwrap();

        let chunk = match inner.chunk_sections.get_mut(&location.to_chunk_section_pos().map_numeric_hash()) {
            Some(chunk) => chunk,
            None => {
                inner.chunk_sections.insert(
                    location.to_chunk_section_pos().map_numeric_hash(), 
                    ChunkSection::default()
                );
                inner.chunk_sections.get_mut(&location.to_chunk_section_pos().map_numeric_hash()).unwrap()
            },
        };

        BlockState::from_protocol(
            voxidian_protocol::value::BlockState::from_id(
                chunk.block_at(location.x as usize % 15, location.y as usize % 15, location.z as usize % 15).id() as i32
            ).unwrap()
        )
    }
}

impl Hash for Dimension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.lock().unwrap().name.hash(state);
    }
}

impl PartialEq for Dimension {
    fn eq(&self, other: &Self) -> bool {
        self.inner.lock().unwrap().name == other.inner.lock().unwrap().name
    }
}

impl Eq for Dimension {}
