use super::ModuleVar;
use crate::config::MemoryConfig;
use std::borrow::Cow;
use sysinfo::System;

const DEFAULT_UNIT: &str = "Mib";
const BYTES_IN_KILOBYTES: u64 = 1024;
const BYTES_IN_MEGABYTES: u64 = 1000000;
const BYTES_IN_GIGABYTES: u64 = 1000000000;

pub struct MemoryVar<'a> {
    pub name: &'a str,
}

impl<'a> ModuleVar<MemoryConfig<'a>> for MemoryVar<'a> {
    fn new() -> Self {
        Self { name: "memory" }
    }

    fn value(self, cfg: Option<&MemoryConfig>) -> String {
        let config = cfg.unwrap();

        // Try evit a shared reference to move this clone
        let unit = &config.unit.clone().unwrap_or(Cow::from(DEFAULT_UNIT));
        let mut sys = System::new();
        sys.refresh_memory();

        let total_memory = bytes_to(sys.total_memory(), unit);
        let used_memory = bytes_to(sys.used_memory(), unit);

        if config.percent.unwrap_or(true) {
            let percent = (used_memory / total_memory) * 100.0;
            return format!(
                "{}{unit} / {}{unit} ({}%)",
                used_memory, total_memory, percent as u64
            );
        }

        format!("{}{unit} / {}{unit}", used_memory, total_memory)
    }
}

fn bytes_to(bytes: u64, format: &str) -> f64 {
    match format.to_lowercase().as_str() {
        "kib" => (bytes / BYTES_IN_KILOBYTES) as f64,
        "mib" => (bytes / BYTES_IN_MEGABYTES) as f64,
        "gib" => (bytes / BYTES_IN_GIGABYTES) as f64,
        bad_format => {
            eprintln!(
                "Error: {} is not a valid format for show memory, use one of 'Kib', 'Mib' or 'Gib' instead",
                bad_format
            );
            std::process::exit(1)
        }
    }
}
