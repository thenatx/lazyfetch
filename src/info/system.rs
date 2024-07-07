use crate::config::Os;
use sysinfo::System;

pub fn os(config: &Os) -> String {
    let name = System::name().unwrap();
    let version = if config.shorthand.unwrap_or_default() {
        System::os_version().unwrap()
    } else {
        System::long_os_version().unwrap()
    };

    if config.show_arch.unwrap_or_default() {
        let os_arch = System::cpu_arch().unwrap();
        return format!("{} {} {}", name, version, os_arch);
    }

    format!("{} {}", name, version)
}
