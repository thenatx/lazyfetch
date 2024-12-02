use super::ModuleVar;
use crate::{config::file::MemoryConfig, error::LazyfetchError};
use std::borrow::Cow;
use sysinfo::System;

const DEFAULT_UNIT: &str = "Mib";
const BYTES_IN_KILOBYTES: u64 = 1024;
const BYTES_IN_MEGABYTES: u64 = 1_000_000;
const BYTES_IN_GIGABYTES: u64 = 1_000_000_000;

pub struct MemoryVar;

impl ModuleVar<MemoryConfig<'_>> for MemoryVar {
    fn name(self) -> String {
        String::from("memory")
    }

    fn value(self, cfg: Option<&MemoryConfig>) -> Result<String, LazyfetchError> {
        let config = cfg.unwrap();

        // TODO: try evit a shared reference to move this clone
        let unit = &config.unit.clone().unwrap_or(Cow::from(DEFAULT_UNIT));
        let mut sys = System::new();
        sys.refresh_memory();

        let total_memory = bytes_to(sys.total_memory(), unit);
        let used_memory = bytes_to(sys.used_memory(), unit);

        if config.percent.unwrap_or(true) {
            let percent = (used_memory / total_memory) * 100.0;
            return Ok(format!(
                "{}{unit} / {}{unit} ({}%)",
                used_memory, total_memory, percent as u64
            ));
        }

        Ok(format!("{}{unit} / {}{unit}", used_memory, total_memory))
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
