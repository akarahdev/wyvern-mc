use std::{fmt::Debug, pin::Pin, sync::Arc};

use crate::{
    dimension::{Dimension, blocks::BlockState},
    inventory::ItemStack,
    player::Player,
    server::Server,
    values::{Vec2, Vec3},
};
use futures_util::future::join_all;

macro_rules! event_bus {
    ($($name:ident : $t:ty)*) => {
        #[derive(Default)]
        pub struct EventBus {
            $(pub(crate) $name: Vec<Arc<Box<dyn Fn(Arc<$t>) -> BoxedFuture + Send + Sync>>>,)*
        }

        $(impl crate::events::Event for $t {
            fn add_handler(bus: &mut EventBus, f: Box<dyn Fn(Arc<$t>) -> BoxedFuture + Send + Sync>) {
                bus.$name.push(Arc::new(f));
            }

            fn dispatch(self, bus: std::sync::Arc<EventBus>) {
                crate::runtime::Runtime::spawn(async move {
                    self.dispatch_sync(bus).await;
                });
            }

            async fn dispatch_sync(self, bus: Arc<EventBus>) {
                let tick1 = std::time::Instant::now();
                let tick2 = std::time::Instant::now();
                let time_time = tick2 - tick1;
                let start = std::time::Instant::now();
                let event = Arc::new(self);
                let futures_to_poll = bus
                        .$name
                        .clone()
                        .into_iter()
                        .map(|x| x(event.clone()));
                    join_all(futures_to_poll).await;
                let end = std::time::Instant::now();
                log::debug!("Event {:?} took {:?} to execute", std::any::type_name::<Self>(), (end - start) - (time_time));
            }
        })*

    };
}

event_bus! {
    on_dim_create: DimensionCreateEvent
    on_server_tick: ServerTickEvent
    on_player_move: PlayerMoveEvent
    on_chunk_load: ChunkLoadEvent
    on_command: PlayerCommandEvent
    on_server_start: ServerStartEvent
    on_place_block: PlaceBlockEvent
    on_change_held_slot: ChangeHeldSlotEvent
    on_swap_hands: SwapHandsEvent
    on_drop_item: DropItemEvent
    on_block_break: BreakBlockEvent
    on_chat: ChatMessageEvent
}

impl Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EventBus { ... }")
    }
}

pub trait Event {
    fn add_handler(bus: &mut EventBus, f: Box<dyn Fn(Arc<Self>) -> BoxedFuture + Send + Sync>);
    fn dispatch(self, bus: Arc<EventBus>);
    #[allow(async_fn_in_trait)] // allowed since this method is auto generated by the event_bus! macro
    async fn dispatch_sync(self, bus: Arc<EventBus>);
}

pub type BoxedFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

#[derive(Debug, Clone)]
pub struct DimensionCreateEvent {
    pub dimension: Dimension,
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct ChunkLoadEvent {
    pub dimension: Dimension,
    pub pos: Vec2<i32>,
}

#[derive(Debug, Clone)]
pub struct ServerTickEvent {
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct PlayerMoveEvent {
    pub player: Player,
    pub new_position: Vec3<f64>,
    pub new_direction: Vec2<f32>,
}

#[derive(Debug, Clone)]
pub struct PlayerCommandEvent {
    pub player: Player,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct StartBreakBlockEvent {
    pub player: Player,
    pub position: Vec3<i32>,
}

#[derive(Debug, Clone)]
pub struct StopBreakBlockEvent {
    pub player: Player,
    pub position: Vec3<i32>,
}

#[derive(Debug, Clone)]
pub struct BreakBlockEvent {
    pub player: Player,
    pub position: Vec3<i32>,
}

#[derive(Debug, Clone)]
pub struct PlaceBlockEvent {
    pub player: Player,
    pub position: Vec3<i32>,
    pub block: BlockState,
}

#[derive(Debug, Clone)]
pub struct DropItemEvent {
    pub player: Player,
    pub item: ItemStack,
}

#[derive(Debug, Clone)]
pub struct SwapHandsEvent {
    pub player: Player,
}

#[derive(Debug, Clone)]
pub struct ChangeHeldSlotEvent {
    pub player: Player,
    pub slot: i16,
}

#[derive(Debug, Clone)]
pub struct ServerStartEvent {
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct ChatMessageEvent {
    pub player: Player,
    pub message: String,
}
