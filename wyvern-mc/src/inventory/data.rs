use std::sync::{Arc, Mutex};

use super::{Inventory, ItemStack, RootInventoryData};

pub struct DataInventory<const S: usize> {
    pub(crate) slots: Arc<Mutex<RootInventoryData<S>>>
}

impl<const S: usize> DataInventory<S> {
    pub fn new() -> DataInventory<S> {
        DataInventory {
            slots: Arc::new(Mutex::new(RootInventoryData::default()))
        }
    }
}

impl<const S: usize> Inventory for DataInventory<S> {
    fn get_slot(&self, slot: usize) -> ItemStack {
        if slot >= S {
            return ItemStack::air();
        }
        self.slots.lock().unwrap().slots[slot].clone()
    }

    fn set_slot(&mut self, slot: usize, stack: ItemStack) {
        if slot >= S { return; }
        self.slots.lock().unwrap().slots[slot] = stack;
    }
}