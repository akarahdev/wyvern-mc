use voxidian_protocol::{registry::RegEntry, value::{BlockState, ChunkSection as ProtocolSection, PaletteFormat, PalettedContainer}};

pub(crate) struct ChunkSection {
    pub(crate) block_count: i16,
    pub(crate) blocks: [[[RegEntry<BlockState>; 16]; 16]; 16]
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
    pub(crate) fn flatten_blocks(&self) -> [RegEntry<BlockState>; 4096] {
        let mut arr = [unsafe { RegEntry::new_unchecked(0) }; 4096];
        let mut idx = 0;
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    arr[idx] = self.blocks[y][z][x];
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