use crate::config::Os;
use sysinfo::System;

pub fn os(config: Os) -> String {
    let name = System::name().unwrap();
    let version = if !cfg!(target_os = "linux") {
        System::os_version().unwrap()
    } else {
        let distro_info = get_distro_info();
        let mut codename = distro_info.codename.unwrap_or(String::new());
        codename = codename.replace(&codename[0..1], &codename[0..1].to_uppercase());

        if config.shorthand.unwrap_or_default() {
            format!("{}", distro_info.version.unwrap())
        } else {
            format!("{} ({})", distro_info.build_id.unwrap(), codename)
        }
    };

    if config.show_arch.unwrap_or_default() {
        let os_arch = System::cpu_arch().unwrap();
        return format!("{} {} {}", name, version, os_arch);
    }

    format!("{} {}", name, version)
}

fn get_distro_info() -> LinuxOsRelease {
    let info_lines = std::fs::read_to_string("/etc/os-release").unwrap();

    let mut info: LinuxOsRelease = Default::default();
    for line in info_lines.split("\n") {
        let item: Vec<&str> = line.splitn(2, '=').collect();
        if item.len() < 2 {
            break;
        }

        let line_key = item[0];
        let line_value = if item[1].contains('"') {
            let value = item[1];
            String::from(&value[1..value.len() - 1])
        } else {
            String::from(item[1])
        };

        match (line_key, line_value) {
            ("NAME", val) => info.name = Some(val),
            ("PRETTY_NAME", val) => info.pretty_name = Some(val),
            ("VERSION", val) => info.version = Some(val),
            ("ANSI_COLOR", val) => info.ansi_color = Some(val),
            ("BUILD_ID", val) => info.build_id = Some(val),
            ("VERSION_CODENAME", val) => info.codename = Some(val),
            _ => {}
        }
    }

    info
}

struct LinuxOsRelease {
    name: Option<String>,
    pretty_name: Option<String>,
    version: Option<String>,
    build_id: Option<String>,
    codename: Option<String>,
    ansi_color: Option<String>,
}

impl Default for LinuxOsRelease {
    fn default() -> Self {
        Self {
            name: None,
            pretty_name: None,
            version: None,
            build_id: None,
            codename: None,
            ansi_color: None,
        }
    }
}
