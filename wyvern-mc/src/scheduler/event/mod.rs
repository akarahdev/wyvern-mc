use std::ops::Deref;

use crate::{dimension::Dimension, inventory::ItemStack, values::{BlockPosition, Location}, Player};
use super::{parameters::TaskParameter, TypeMap};

mod all_events;
pub use all_events::*;

#[derive(Clone)]
pub struct Event<E: EventFetcher> {
    event: E
}

impl<E: EventFetcher> Event<E> {
    pub fn new(event: E) -> Self {
        Event { event }
    }
}

impl<E: EventFetcher> Deref for Event<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

pub trait EventFetcher: Clone + Sized + 'static {
    fn from_data(map: TypeMap) -> Option<Self> {
        map.get().cloned()
    }
}





pub trait EventParameter: Clone {}
impl EventParameter for Player {}
impl EventParameter for Dimension {}
impl EventParameter for Location {}
impl EventParameter for BlockPosition {}
impl EventParameter for ItemStack {}
impl EventParameter for usize {}

#[derive(Clone)]
pub struct Param<P: EventParameter> {
    parameter: P
}

impl<P: EventParameter + 'static> Param<P> {
    pub fn new(parameter: P) -> Self {
        Param { parameter }
    }
}

impl<P: EventParameter> Deref for Param<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.parameter
    }
}

impl<P: EventParameter + 'static> TaskParameter for Param<P> {
    fn fetch(data: &TypeMap) -> Option<Self> {
        data.get::<Param<P>>().cloned()
    }
}

impl<E: EventFetcher + 'static> TaskParameter for Event<E> {
    fn fetch(data: &TypeMap) -> Option<Self> {
        data.get::<Event<E>>().cloned()
    }
}