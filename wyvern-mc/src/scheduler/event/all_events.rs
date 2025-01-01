use super::EventFetcher;


#[derive(Clone)]
pub struct ConnectEvent;
impl EventFetcher for ConnectEvent {}

#[derive(Clone)]
pub struct PlayerTickEvent;
impl EventFetcher for PlayerTickEvent {}

#[derive(Clone)]
pub struct MoveEvent;
impl EventFetcher for MoveEvent {}

#[derive(Clone)]
pub struct SneakEvent;
impl EventFetcher for SneakEvent {}

#[derive(Clone)]
pub struct SprintEvent;
impl EventFetcher for SprintEvent {}