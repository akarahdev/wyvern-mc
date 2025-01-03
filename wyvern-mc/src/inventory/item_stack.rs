
use voxidian_protocol::value::{Identifier, Item, SlotData, VarInt};

use crate::values::Key;

#[derive(Debug, Clone)]
pub struct ItemStack {
    key: Key<ItemStack>,
    count: u8
}

impl ItemStack {
    pub(crate) fn from_slot_data(_data: &SlotData) -> ItemStack {
        let item = Item::vanilla_registry().lookup(&_data.item_id).unwrap().id.clone();
        let item = Key::<ItemStack>::new(item.namespace, item.path);

        ItemStack {
            key: item,
            count: _data.item_count.as_i32() as u8
        }
    }

    pub(crate) fn to_slot_data(&self) -> SlotData {
        let entry = Item::vanilla_registry().make_entry(&Identifier::new(&self.key.namespace, &self.key.path)).unwrap();

        SlotData {
            item_count: VarInt::from(self.count as i32),
            item_id: entry,
            components: vec![],
            removed_components: vec![],
        }
    }
}

impl ItemStack {
    pub fn new(key: Key<ItemStack>) -> ItemStack {
        ItemStack {
            key,
            count: 1
        }
    }
    
    pub fn air() -> ItemStack {
        ItemStack::new(Key::new("minecraft", "air"))
    }

    pub fn map<F: FnOnce(&mut ItemStack)>(&mut self, f: F) -> &mut Self {
        f(self);
        self
    }

    pub fn set_count(&mut self, count: u8) {
        assert!(count <= 99);
        self.count = count;
    }
}