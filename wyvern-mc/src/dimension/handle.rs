use std::{collections::HashMap, sync::{Arc, Mutex}};

use voxidian_protocol::{packet::s2c::play::{BlockUpdateS2CPlayPacket, WorldChunkWithLightS2CPlayPacket}, registry::RegEntry, value::{BlockPos, BlockState as ProtocolBlockState, ChunkSectionData, Nbt, NbtCompound}};

use crate::{values::{BlockPosition, Key, Location, Position}, Server};

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

    pub(crate) fn get_chunk_as_packets(&self, cp: BlockPosition) -> WorldChunkWithLightS2CPlayPacket {
        let mut inner = self.inner.lock().unwrap();

        WorldChunkWithLightS2CPlayPacket {
            chunk_x: cp.x() as i32,
            chunk_z: cp.z() as i32,
            heightmaps: Nbt { name: "".to_string(), root: NbtCompound::new() },
            data: ChunkSectionData {
                sections: {
                    let mut sections = Vec::new();
                    for chunk_y in (inner.min_y..inner.max_y).step_by(16) {
                        let cp = BlockPosition::new(cp.x() as i32, chunk_y, cp.z() as i32);
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

    pub fn set_block(&mut self, location: Location, block: BlockState) {
        let centered = location.center();
        let bp = BlockPosition::new(centered.x().floor() as i32, centered.y().floor() as i32, centered.z().floor() as i32);
        let cp = BlockPosition::new(bp.x() as i32 / 16, bp.y() as i32 / 16, bp.z() as i32 / 16);
        let mut inner = self.inner.lock().unwrap();
        let chunk = match inner.chunk_sections.get_mut(&cp) {
            Some(chunk) => chunk,
            None => {
                inner.chunk_sections.insert(cp, ChunkSection::default());
                inner.chunk_sections.get_mut(&cp).unwrap()
            },
        };
        let rg = unsafe { RegEntry::new_unchecked(block.to_protocol().to_id().unwrap() as usize) };
        let cbp = BlockPosition::new(bp.x() as i32 % 16, bp.y() as i32 % 16, bp.z() as i32 % 16);
        let old_rg = chunk.blocks[cbp.y() as usize][cbp.z() as usize][cbp.x() as usize];
        chunk.blocks[cbp.y() as usize][cbp.z() as usize][cbp.x() as usize] = rg;

        if rg.id() == 0 && rg.id() != old_rg.id() {
            chunk.block_count -= 1;
        }
        if rg.id() != 0 && old_rg.id() == 0 {
            chunk.block_count += 1;
        }
        for conn in Server::get().connections() {
            conn.protocol_handle().send_packet(BlockUpdateS2CPlayPacket {
                pos: BlockPos::new(centered.x().floor() as i32, centered.y().floor() as i32, centered.z().floor() as i32),
                block: rg,
            }).unwrap();
        }
    }

    pub fn get_block(&mut self, location: Location) -> BlockState {
        let centered = location.center();
        let bp = BlockPosition::new(centered.x().floor() as i32, centered.y().floor() as i32, centered.z().floor() as i32);
        let cp = BlockPosition::new(bp.x() as i32 / 16, bp.y() as i32 / 16, bp.z() as i32 / 16);
        let mut inner = self.inner.lock().unwrap();
        let chunk = match inner.chunk_sections.get_mut(&cp) {
            Some(chunk) => chunk,
            None => {
                inner.chunk_sections.insert(cp, ChunkSection::default());
                inner.chunk_sections.get_mut(&cp).unwrap()
            },
        };
        let cbp = BlockPosition::new(bp.x() as i32 % 16, bp.y() as i32 % 16, bp.z() as i32 % 16);
        BlockState::from_protocol(
            ProtocolBlockState::from_id(chunk.blocks[cbp.y() as usize][cbp.z() as usize][cbp.x() as usize].id() as i32).unwrap()
        )
    }
}