pub mod primitives;

use primitives::{
    configuration::ConfigurationPlugin, handshake::HandshakePlugin, login::LoginPlugin,
    play::PlayPlugin, status::StatusPlugin,
};

use crate::{Server, ServerBuilder};

pub trait Plugin {
    fn load(&self, server: &ServerBuilder);
}

pub struct Setup;

impl Plugin for Setup {
    fn load(&self, server: &ServerBuilder) {
        server
            .add_plugin(HandshakePlugin)
            .add_plugin(LoginPlugin)
            .add_plugin(StatusPlugin)
            .add_plugin(ConfigurationPlugin)
            .add_plugin(PlayPlugin);
    }
}
