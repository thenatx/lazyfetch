use crate::config::CpuConfig;

use sysinfo::System;

use std::fs::read_to_string;

use super::ModuleVar;

const CPU_FREQ_BASE_DIR: &str = "/sys/devices/system/cpu/cpu0/cpufreq/";

pub struct CpuVar<'a> {
    pub name: &'a str,
}

impl ModuleVar<CpuConfig> for CpuVar<'_> {
    fn new() -> Self {
        Self { name: "cpu" }
    }
    fn value(self, cfg: Option<&CpuConfig>) -> String {
        let config = cfg.unwrap();

        let mut sys = System::new_all();
        sys.refresh_all();
        let cpu_brand = if config.show_brand.unwrap_or(true) {
            sys.cpus()[0].brand()
        } else {
            ""
        };

        #[cfg(target_os = "linux")]
        if config.show_speed.unwrap_or(true) {
            let speed_type = &config
                .speed_type
                .clone()
                .unwrap_or("bios_limit".to_string());
            let cpu_freq = match read_to_string(format!("{}{}", CPU_FREQ_BASE_DIR, speed_type)) {
                Ok(val) => {
                    let val = val.replace("\n", "");
                    let cpu_freq: f64 = val.parse().unwrap();

                    cpu_freq / 1000000.0
                }

                Err(_) => {
                    eprintln!(
                        "Error: your speed_type no exists in the {} directory",
                        CPU_FREQ_BASE_DIR
                    );
                    std::process::exit(1)
                }
            };

            return format!("{} @ {}GHz", cpu_brand, cpu_freq);
        }

        if !config.show_brand.unwrap_or(true) && !config.show_speed.unwrap_or(true) {
            eprintln!("Why you want this void?\n show speed and show brand options are disabled");
            std::process::exit(1)
        }

        cpu_brand.to_string()
    }
}
