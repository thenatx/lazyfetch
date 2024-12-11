use std::fs;
use sysinfo::System;

use crate::error::LazyfetchError;

use super::ModuleVar;

pub struct HostNameVar;

impl ModuleVar<!> for HostNameVar {
    fn name(self) -> String {
        String::from("hostname")
    }
    fn value(self, _cfg: Option<&!>) -> Result<String, LazyfetchError> {
        Ok(System::host_name().unwrap())
    }
}

pub struct HostVar;

impl ModuleVar<!> for HostVar {
    fn name(self) -> String {
        String::from("host")
    }
    fn value(self, _cfg: Option<&!>) -> Result<String, LazyfetchError> {
        // TODO: Support other systems that aren't GNU/linux based
        if let Ok(family) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family") {
            return Ok(delete_end_extraspace(family));
        };
        if let Ok(family) = fs::read_to_string("/sys/class/dmi/id/product_family") {
            return Ok(delete_end_extraspace(family));
        };

        if let Ok(name) = fs::read_to_string("/sys/class/dmi/id/product_name") {
            return Ok(delete_end_extraspace(name));
        };

        if let Ok(name) = fs::read_to_string("/sys/class/dmi/id/product_version") {
            return Ok(delete_end_extraspace(name));
        };

        Err(LazyfetchError::Custom(
            "Error trying to get the host of your system seems like you are on an unsupported one"
                .to_string(),
        ))
    }
}

fn delete_end_extraspace(content: String) -> String {
    if content.ends_with('\n') {
        return content.replace('\n', "");
    }

    content
}
