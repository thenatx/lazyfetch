use super::ModuleVar;
use crate::error;

pub struct Os<'a> {
    name: &'a str, // Name of the var to use un config (like in "Hello from ${os}")
    value: String, // This value is used to replace var while parsing, for example: "${os}" to "Arch linux"
}

impl<'a> ModuleVar for Os<'a> {
    fn new() -> Self {
        let name = "os";
        let option_value = sysinfo::System::name();

        Self {
            name,
            value: error::handle_empty_var(option_value),
        }
    }
}
