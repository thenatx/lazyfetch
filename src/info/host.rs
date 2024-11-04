use std::fs;
use sysinfo::System;

use super::ModuleVar;

pub struct HostNameVar;

impl ModuleVar<!> for HostNameVar {
    fn name(self) -> String {
        String::from("hostname")
    }
    fn value(self, _cfg: Option<&!>) -> String {
        System::host_name().expect("Error getting the host name")
    }
}

pub struct HostVar;

impl ModuleVar<!> for HostVar {
    fn name(self) -> String {
        String::from("host")
    }
    fn value(self, _cfg: Option<&!>) -> String {
        // TODO: Support other systems that aren't GNU/linux based
        if let Ok(family) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family") {
            return delete_end_extraspace(family);
        };
        if let Ok(family) = fs::read_to_string("/sys/class/dmi/id/product_family") {
            return delete_end_extraspace(family);
        };

        if let Ok(name) = fs::read_to_string("/sys/class/dmi/id/product_name") {
            return delete_end_extraspace(name);
        };

        if let Ok(name) = fs::read_to_string("/sys/class/dmi/id/product_version") {
            return delete_end_extraspace(name);
        };

        panic!("can't get the host, securely you are in a unsoported system")
    }
}

fn delete_end_extraspace(content: String) -> String {
    if content.ends_with('\n') {
        return content.replace('\n', "");
    }

    return content;
}
