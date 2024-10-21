use super::ModuleVar;
use crate::{config::OsConfig, error};

pub struct OsVar<'a> {
    name: &'a str, // Name of the var to use un config (like in "Hello from ${os}")
    value: String, // This value is used to replace var while parsing, for example: "${os}" to "Arch linux"
}

impl<'a> ModuleVar<OsConfig> for OsVar<'a> {
    fn new() -> Self {
        Self {
            name: "os",
            value: String::new(),
        }
    }

    fn value(&mut self, cfg: Option<OsConfig>) {
        let _cfg: OsConfig = cfg.unwrap();
        let option_value = sysinfo::System::name();
        self.value = error::handle_empty_var(option_value);
    }
}
