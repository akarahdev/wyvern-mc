use std::sync::OnceLock;

pub use voxidian_protocol::{registry::{Registry, RegistryFrozen}, value::{Biome, BlockState, DamageType, Item}};

pub(crate) static SERVER_REGISTRIES: OnceLock<Registries> = OnceLock::new();

pub struct Registries {
    pub(crate) block_states: Vec<BlockState>,
    pub(crate) items: Registry<Item>,
    pub(crate) biomes: Registry<Biome>,
    pub(crate) damage_types: Registry<DamageType>
}

impl Registries {
    pub fn block_states() -> &'static Vec<BlockState> {
        &SERVER_REGISTRIES.get().unwrap().block_states
    }

    pub fn items() -> &'static Registry<Item> {
        &SERVER_REGISTRIES.get().unwrap().items
    }

    pub fn biomes() -> &'static Registry<Biome> {
        &SERVER_REGISTRIES.get().unwrap().biomes
    }

    pub fn damage_types() -> &'static Registry<DamageType> {
        &SERVER_REGISTRIES.get().unwrap().damage_types
    }
}