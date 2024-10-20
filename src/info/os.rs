use super::Module;
use crate::error;

pub struct Os<'a> {
    name: &'a str, // Name of the module to use un config (like in "Hello from ${os}")
    value: String, // This value is used to raplace var while parsing, for example: "${os}" to "Arch linux"
}

impl<'a> Module for Os<'a> {
    fn new() -> Self {
        let name = "os";
        let option_value = sysinfo::System::name();

        Self {
            name,
            value: error::handle_empty_var(option_value),
        }
    }
}
