use voxidian_protocol::{packet::s2c::play::BlockUpdateS2CPlayPacket, registry::RegEntry, value::{BlockPos, BlockState, ChunkSection as ProtocolSection, PaletteFormat, PalettedContainer}};

use crate::Server;

pub struct ChunkSection {
    block_count: i16,
    blocks: [[[RegEntry<BlockState>; 16]; 16]; 16]
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self { 
            block_count: 0,
            blocks: [[[unsafe { RegEntry::new_unchecked(0) }; 16]; 16]; 16]
        }
    }
}

impl ChunkSection {
    pub fn new(
        block_count: i16,
        blocks: [[[RegEntry<BlockState>; 16]; 16]; 16]
    ) -> ChunkSection {
        ChunkSection {
            block_count,
            blocks
        }
    }

    pub fn block_at(&self, x: usize, y: usize, z: usize) -> RegEntry<BlockState> {
        assert!(x <= 15);
        assert!(y <= 15);
        assert!(z <= 15);

        self.blocks[y][z][x]
    }

    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, state: super::BlockState) {
        assert!(x <= 15);
        assert!(y <= 15);
        assert!(z <= 15);

        let original_block = self.block_at(x, y, z);
        let new_block = unsafe { RegEntry::new_unchecked(state.to_protocol().to_id().unwrap() as usize) };
        
        if original_block.id() != 0 && new_block.id() == 0 {
            self.block_count -= 1;
        }
        if new_block.id() != 0 && original_block.id() != new_block.id() {
            self.block_count += 1;
        }

        self.blocks[y][z][x] = new_block;

        for conn in Server::get().connections() {
            conn.protocol_handle().send_packet(BlockUpdateS2CPlayPacket {
                pos: BlockPos::new(x as i32, y as i32, z as i32),
                block: new_block,
            }).unwrap();
        }
    }

    pub fn flatten_blocks(&self) -> [RegEntry<BlockState>; 4096] {
        let mut arr = [unsafe { RegEntry::new_unchecked(0) }; 4096];
        let mut idx = 0;
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    arr[idx] = self.blocks[y][z][x];
                    if arr[idx].id() != 0 {
                        println!("idx: {:?}", arr[idx])
                    }
                    idx += 1;
                }
            }
        }
        arr
    }

    pub(crate) fn to_packet(&self) -> ProtocolSection {
        ProtocolSection {
            block_count: self.block_count,
            block_states: PalettedContainer {
                bits_per_entry: 15,
                format: PaletteFormat::Direct { data: self.flatten_blocks() },
            },
            biomes: PalettedContainer {
                bits_per_entry: 0,
                format: PaletteFormat::SingleValued { entry: unsafe { RegEntry::new_unchecked(0) } }
            },
        }
    }
}