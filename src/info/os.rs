use super::ModuleVar;
use crate::{config::OsConfig, error};

pub struct OsVar<'a> {
    pub name: &'a str, // Name of the var to use un config (like in "Hello from ${os}")
}

impl<'a> ModuleVar<OsConfig> for OsVar<'a> {
    fn new() -> Self {
        Self { name: "os" }
    }

    fn value(&mut self, cfg: Option<&OsConfig>) -> String {
        let cfg: &OsConfig = cfg.unwrap();

        let name = sysinfo::System::name();
        let version = if cfg.shorthand.unwrap_or_default() {
            sysinfo::System::long_os_version()
        } else {
            sysinfo::System::os_version()
        };

        let name = error::handle_empty_var(name);
        let version = error::handle_empty_var(version);

        if cfg.show_arch.unwrap_or_default() {
            format!("{} {}", name, version)
        } else {
            let arch = error::handle_empty_var(sysinfo::System::cpu_arch());
            format!("{} {} ({})", name, version, arch)
        }
    }
}
