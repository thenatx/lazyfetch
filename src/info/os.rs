use super::ModuleVar;
use crate::{config::OsConfig, error::LazyfetchError};

pub struct OsVar;

impl ModuleVar<OsConfig> for OsVar {
    fn name(self) -> String {
        String::from("os")
    }
    fn value(self, cfg: Option<&OsConfig>) -> Result<String, LazyfetchError> {
        let cfg = cfg.unwrap();

        let name = sysinfo::System::name().unwrap();
        let version = if cfg.shorthand.unwrap_or_default() {
            sysinfo::System::long_os_version().unwrap()
        } else {
            sysinfo::System::os_version().unwrap()
        };

        if cfg.show_arch.unwrap_or_default() {
            Ok(format!("{} {}", name, version))
        } else {
            let arch = sysinfo::System::cpu_arch().unwrap();
            Ok(format!("{} {} ({})", name, version, arch))
        }
    }
}
