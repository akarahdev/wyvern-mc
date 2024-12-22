pub mod login;

use crate::Server;

pub trait Plugin {
    fn load(&self, server: &mut Server);
}