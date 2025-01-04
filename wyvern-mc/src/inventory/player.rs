use std::sync::{atomic::{AtomicU8, Ordering}, Arc, Mutex};

use crate::{Player, WeakRefPlayer};

use super::{Inventory, ItemStack, RootInventoryData};

#[derive(Clone)]
pub struct PlayerInventory {
    pub(crate) inner: Arc<Mutex<RootInventoryData<45>>>,
    pub(crate) held_slot_in_hotbar: Arc<AtomicU8>,
    pub(crate) player: WeakRefPlayer,
    pub(crate) inventory_offset: Arc<AtomicU8> // this field exists because mojank's inventory system is weird
}

pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Head,
    Chest,
    Legs,
    Boots
}

impl Inventory for PlayerInventory {
    fn get_slot(&self, slot: usize) -> ItemStack {
        self.inner.lock().unwrap().slots.get(slot).unwrap().clone()
    }

    fn set_slot(&mut self, slot: usize, stack: ItemStack) {
        self.inner.lock().unwrap().slots[slot] = stack;
        
        self.player.upgrade().inspect(|_player| {
            // TODO: send slot set packet
        });
    }
}

impl PlayerInventory {
    pub fn new(player: Player) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RootInventoryData::default())),
            held_slot_in_hotbar: Arc::new(AtomicU8::new(0)),
            player: player.make_weak(),
            inventory_offset: Arc::new(AtomicU8::new(0))
        }
    }

    pub(crate) fn set_slot_in_memory(&self, slot: usize, stack: ItemStack) {
        self.inner.lock().unwrap().slots[slot] = stack;
    }

    pub fn set_held_slot(&self, value: u8) {
        self.held_slot_in_hotbar.store(value, Ordering::Relaxed);
        // TODO: send change hotbar slot packet
    }

    pub fn set_held_slot_in_memory(&self, value: u8) {
        self.held_slot_in_hotbar.store(value, Ordering::Relaxed);
    }

    pub fn get_held_slot(&self) -> u8 {
        self.held_slot_in_hotbar.load(Ordering::Relaxed)
    }

    pub fn get_slot_id(&self, slot: EquipmentSlot) -> usize {
        match slot {
            EquipmentSlot::MainHand => 36 + self.held_slot_in_hotbar.load(Ordering::Relaxed),
            EquipmentSlot::OffHand => 39,
            EquipmentSlot::Head => 5,
            EquipmentSlot::Chest => 6,
            EquipmentSlot::Legs => 7,
            EquipmentSlot::Boots => 8
        }.into()
    }

    pub fn get_equipment_slot(&self, slot: EquipmentSlot) -> ItemStack {
        self.get_slot(self.get_slot_id(slot))
    }

    pub fn set_equipment_slot(&mut self, slot: EquipmentSlot, stack: ItemStack) {
        self.set_slot(self.get_slot_id(slot), stack);
    }
}