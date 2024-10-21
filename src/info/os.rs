use super::ModuleVar;
use crate::{config::OsConfig, error};

pub struct OsVar<'a> {
    name: &'a str, // Name of the var to use un config (like in "Hello from ${os}")
    value: String, // This value is used to replace var while parsing, for example: "${os}" to "Arch linux"
}

impl<'a> ModuleVar<OsConfig> for OsVar<'a> {
    fn new(config: Option<OsConfig>) -> Self {
        let _config = config.unwrap();
        let name = "os";
        let option_value = sysinfo::System::name();

        Self {
            name,
            value: error::handle_empty_var(option_value),
        }
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}
