use std::collections::HashMap;

use voxidian_protocol::value::{BlockState as ProtocolBlockState, Identifier};

use crate::values::Key;

pub struct Block;

pub struct BlockState {
    block_id: Key<Block>,
    properties: HashMap<String, String>
}

impl BlockState {
    pub fn new(material: Key<Block>) -> Self {
        BlockState {
            block_id: material,
            properties: HashMap::new()
        }
    }

    pub fn set_property<T: ToString>(&mut self, key: &str, value: T) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }

    pub(crate) fn to_protocol(&self) -> ProtocolBlockState {
        ProtocolBlockState {
            id: Identifier::new(&self.block_id.namespace, &self.block_id.path),
            properties: self
                .properties
                .iter()
                .map(|x| (x.0.clone(), x.1.clone()))
                .collect::<Vec<_>>(),
        }
    }

    pub(crate) fn from_protocol(ptb: ProtocolBlockState) -> Self {
        BlockState {
            block_id: Key::new(ptb.id.namespace, ptb.id.path),
            properties: {
                let mut map = HashMap::new();
                for entry in ptb.properties {
                    map.insert(entry.0, entry.1);
                }
                map
            }
        }
    }
}