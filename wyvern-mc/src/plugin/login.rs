use crate::Server;
use crate::plugin::Plugin;

pub struct LoginProtocol;

impl Plugin for LoginProtocol {
    fn load(&self, server: &mut Server) {}
}
