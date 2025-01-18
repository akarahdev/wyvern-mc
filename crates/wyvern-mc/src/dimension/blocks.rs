use voxidian_protocol::autogenerated::block_states::BLOCK_STATE_TO_ID;
use voxidian_protocol::value::BlockState as ProtocolState;

use crate::values::key::Key;

pub struct Block {}

pub struct BlockState {
    block: Key<Block>,
    state: Vec<(String, String)>,
}

impl BlockState {
    pub fn protocol_id(&self) -> i32 {
        *BLOCK_STATE_TO_ID.get(&self.into()).unwrap()
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
