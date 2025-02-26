use std::str::FromStr;

use voxidian_protocol::autogenerated::block_states::{
    BLOCK_STATE_DEFAULTS, BLOCK_STATE_TO_ID, ID_TO_BLOCK_STATE,
};
use voxidian_protocol::value::BlockState as ProtocolState;

use crate::values::Key;

use super::properties::StateProperty;

pub struct Block {}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockState {
    pub(crate) block: Key<Block>,
    pub(crate) state: Vec<(String, String)>,
}

impl BlockState {
    pub fn new(id: Key<Block>) -> BlockState {
        BlockState {
            block: id,
            state: Vec::new(),
        }
        .make_valid()
    }

    pub fn name(&self) -> &Key<Block> {
        &self.block
    }

    pub(crate) fn insert_raw_property(&mut self, key: &str, value: &str) {
        if let Some(index) = self.state.iter().map(|x| &x.0).position(|x| x == key) {
            self.state.remove(index);
        }
        self.state.push((key.into(), value.into()));
    }

    pub fn with_property<T: ToString + FromStr>(
        mut self,
        property: StateProperty<T>,
        value: T,
    ) -> Self {
        self.insert_raw_property(property.name, &value.to_string());
        self
    }

    pub fn protocol_id(&self) -> i32 {
        *BLOCK_STATE_TO_ID.get(&self.into()).unwrap_or(&0)
    }

    pub fn from_protocol_id(id: i32) -> Self {
        Self::from(
            ID_TO_BLOCK_STATE
                .get(&id)
                .unwrap_or(ID_TO_BLOCK_STATE.get(&0).unwrap()),
        )
    }

    pub fn id_is_valid(&self) -> bool {
        BLOCK_STATE_DEFAULTS.contains_key(&self.block.clone().into())
    }

    pub fn make_valid(self) -> Self {
        let mut underlying = ProtocolState {
            id: self.block.into(),
            properties: self.state,
        };
        let _ = underlying.make_valid();
        BlockState {
            block: underlying.id.into(),
            state: underlying.properties,
        }
    }
}

impl From<&ProtocolState> for BlockState {
    fn from(value: &ProtocolState) -> Self {
        BlockState {
            block: value.id.clone().into(),
            state: value.properties.clone(),
        }
    }
}

impl From<&BlockState> for ProtocolState {
    fn from(value: &BlockState) -> Self {
        ProtocolState {
            id: value.block.clone().into(),
            properties: value.state.clone(),
        }
    }
}

pub struct Blocks;
wyvern_macros::generate_blocks_types!();
