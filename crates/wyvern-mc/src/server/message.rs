use std::sync::Arc;

use tokio::sync::oneshot::Sender;
use voxidian_protocol::{
    registry::Registry,
    value::{Biome, DamageType, DimType, PaintingVariant, WolfVariant},
};

use crate::{player::player::ConnectionWithSignal, systems::typemap::TypeMap};

pub enum ServerMessage {
    SpawnConnection(ConnectionWithSignal),
    FireSystems(TypeMap),

    DamageTypeRegistry(Sender<Arc<Registry<DamageType>>>),
    BiomeRegistry(Sender<Arc<Registry<Biome>>>),
    WolfRegistry(Sender<Arc<Registry<WolfVariant>>>),
    PaintingRegistry(Sender<Arc<Registry<PaintingVariant>>>),
    DimTypeRegistry(Sender<Arc<Registry<DimType>>>),
}
