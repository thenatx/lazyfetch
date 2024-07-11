use crate::config::Memory;

use sysinfo::System;

const DEFAULT_UNIT: &'static str = "Mib";

pub fn get_info(config: &Memory) -> String {
    let unit = config.unit.clone().unwrap_or(DEFAULT_UNIT.to_string());
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = bytes_to(sys.total_memory(), &unit);
    let used_memory = bytes_to(sys.used_memory(), &unit);

    if config.percent.unwrap_or(true) {
        let percent = (used_memory as f64 / total_memory as f64) * 100.0;
        return format!(
            "{}{unit} / {}{unit} ({}%)",
            used_memory, total_memory, percent as u64
        );
    }

    format!("{}{unit} / {}{unit}", used_memory, total_memory)
}

fn bytes_to(bytes: u64, format: &str) -> u64 {
    const BYTES_IN_KILOBYTES: u64 = 1024;
    const BYTES_IN_MEGABYTES: u64 = 1000000;
    const BYTES_IN_GIGABYTES: u64 = 1000000000;
    match format {
        "Kib" => bytes / BYTES_IN_KILOBYTES,
        "Mib" => bytes / BYTES_IN_MEGABYTES,
        "Gib" => bytes / BYTES_IN_GIGABYTES,
        bad_format => {
            eprintln!(
                "Error: {} is not a valid format, use one of 'Kib', 'Mib' or 'Gib' instead",
                bad_format
            );
            panic!()
        }
    }
}
