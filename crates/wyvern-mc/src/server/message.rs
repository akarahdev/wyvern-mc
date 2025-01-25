use std::sync::Arc;

use tokio::sync::oneshot::Sender;
use voxidian_protocol::{
    registry::Registry,
    value::{Biome, DamageType, DimType, PaintingVariant, WolfVariant},
};

use crate::{
    dimension::Dimension,
    player::{ConnectionWithSignal, Player},
    systems::typemap::TypeMap,
    values::Key,
};

pub enum ServerMessage {
    SpawnConnection(ConnectionWithSignal),
    FireSystems(TypeMap),

    DamageTypeRegistry(Sender<Arc<Registry<DamageType>>>),
    BiomeRegistry(Sender<Arc<Registry<Biome>>>),
    WolfRegistry(Sender<Arc<Registry<WolfVariant>>>),
    PaintingRegistry(Sender<Arc<Registry<PaintingVariant>>>),
    DimTypeRegistry(Sender<Arc<Registry<DimType>>>),

    GetDimension(Key<Dimension>, Sender<Option<Dimension>>),

    GetConnections(Sender<Vec<Player>>),
}
