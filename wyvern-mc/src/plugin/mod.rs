pub mod login;

use crate::{Server, ServerHandle};

pub trait Plugin {
    fn load(&self, server: ServerHandle);
}
