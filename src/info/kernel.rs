use sysinfo::System;

use crate::error::LazyfetchError;

use super::ModuleVar;

pub struct Kernel;

impl ModuleVar<!> for Kernel {
    fn name(self) -> String {
        String::from("kernel")
    }

    #[cfg(target_os = "linux")]
    fn value(self, _cfg: Option<&!>) -> Result<String, LazyfetchError> {
        let sys = System::kernel_version().unwrap_or("Unknown".to_string());
        Ok(format!("Linux {}", sys))
    }
}
