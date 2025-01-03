use std::array;

pub use item_stack::*;
mod item_stack;
pub use player::*;
mod player;
pub use data::*;
mod data;
pub struct RootInventoryData<const S: usize> {
    pub(crate) slots: [ItemStack; S]
}

impl<const S: usize> Default for RootInventoryData<S> {
    fn default() -> Self {
        Self {
            slots: array::from_fn(|_| ItemStack::air())
        }
    }
}

pub trait Inventory {
    fn get_slot(&self, slot: usize) -> ItemStack;
    fn set_slot(&mut self, slot: usize, stack: ItemStack);
}