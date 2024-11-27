use crate::{config::CpuConfig, error::LazyfetchError};

use sysinfo::System;

use std::fs;

use super::ModuleVar;

const CPU_FREQ_BASE_DIR: &str = "/sys/devices/system/cpu/cpu0/cpufreq/";

pub struct CpuVar;

impl ModuleVar<CpuConfig> for CpuVar {
    fn name(self) -> String {
        String::from("cpu")
    }

    #[cfg(target_os = "linux")]
    fn value(self, cfg: Option<&CpuConfig>) -> Result<String, LazyfetchError> {
        let config = cfg.unwrap();

        let mut sys = System::new_all();
        sys.refresh_all();
        let cpu_brand = if config.show_brand.unwrap_or(true) {
            sys.cpus()[0].brand()
        } else {
            ""
        };

        if config.show_speed.unwrap_or(true) {
            let speed_type = &config
                .speed_type
                .clone()
                .unwrap_or("bios_limit".to_string());
            let cpu_freq = fs::read_to_string(format!("{}{}", CPU_FREQ_BASE_DIR, speed_type))?
                .replace("\n", "")
                .parse::<f32>()
                .unwrap()
                / 1000000.0;

            return Ok(format!("{} @ {}GHz", cpu_brand, cpu_freq));
        }

        if !config.show_brand.unwrap_or(true) && !config.show_speed.unwrap_or(true) {
            eprintln!("Why you want this void?\n show speed and show brand options are disabled");
            std::process::exit(1)
        }

        Ok(cpu_brand.to_string())
    }
}
