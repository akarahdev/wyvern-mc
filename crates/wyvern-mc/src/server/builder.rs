use std::{collections::HashMap, ops::Deref, sync::Arc, time::Instant};

use voxidian_protocol::{
    autogenerated::block_states::{
        BLOCK_STATE_DEFAULTS, BLOCK_STATE_TO_ID, BLOCK_STATES, ID_TO_BLOCK_STATE,
    },
    registry::Registry,
    value::{Biome, DamageType, EntityType},
};

use crate::{
    actors::ActorResult,
    events::{Event, EventBus},
};

use super::{ServerData, dimensions::DimensionContainer, registries::RegistryContainerBuilder};

pub struct ServerBuilder {
    events: EventBus,
    registries: RegistryContainerBuilder,
    dimensions: DimensionContainer,
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerBuilder {
    pub(crate) fn new() -> ServerBuilder {
        ServerBuilder {
            events: EventBus::default(),
            registries: RegistryContainerBuilder {
                damage_types: DamageType::vanilla_registry().into(),
                biomes: Biome::vanilla_registry().into(),
                wolf_variants: Registry::new().into(),
                painting_variants: Registry::new().into(),
                dimension_types: Registry::new().into(),
                entity_types: EntityType::vanilla_registry().into(),
            },
            dimensions: DimensionContainer {
                dimensions: HashMap::new(),
            },
        }
    }

    pub fn event<E: Event + 'static>(mut self, f: fn(Arc<E>) -> ActorResult<()>) -> Self {
        let handler = Box::new(f) as Box<dyn Fn(Arc<E>) -> ActorResult<()> + Send + Sync>;
        E::add_handler(&mut self.events, handler);

        self
    }

    pub fn registries<F: FnOnce(&mut RegistryContainerBuilder)>(mut self, f: F) -> Self {
        f(&mut self.registries);
        self
    }

    pub fn run(self) {
        let chan = flume::unbounded();
        let server = ServerData {
            connections: Vec::new(),
            registries: Arc::new(self.registries.into()),
            dimensions: self.dimensions,
            last_tick: Instant::now(),

            sender: chan.0,
            receiver: chan.1,
            events: Arc::new(self.events),

            last_entity_id: 0,
        };

        log::info!("Initializing some lazy values...");

        let _ = BLOCK_STATES.deref();
        let _ = BLOCK_STATE_DEFAULTS.deref();
        let _ = BLOCK_STATE_TO_ID.deref();
        let _ = ID_TO_BLOCK_STATE.deref();

        server.start();
    }
}
