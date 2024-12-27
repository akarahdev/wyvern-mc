pub mod login;

use crate::ServerHandle;

pub trait Plugin {
    fn load(&self, server: ServerHandle);
}
