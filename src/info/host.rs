use std::fs::read_to_string;
use sysinfo::System;

pub fn host_name() -> String {
    System::host_name().expect("Error getting the host name")
}

// TODO: Support other systems that aren't linux based
pub fn host() -> String {
    if let Ok(mut family) = read_to_string("/sys/devices/virtual/dmi/id/product_family") {
        if family.ends_with("\n") {
            family = family.replace("\n", "");
        };

        return family;
    };
    if let Ok(mut family) = read_to_string("/sys/class/dmi/id/product_family") {
        if family.ends_with("\n") {
            family = family.replace("\n", "");
        };

        return family;
    };

    if let Ok(mut name) = read_to_string("/sys/class/dmi/id/product_name") {
        if name.ends_with("\n") {
            name = name.replace("\n", "");
        };

        return name;
    };

    if let Ok(mut name) = read_to_string("/sys/class/dmi/id/product_version") {
        if name.ends_with("\n") {
            name = name.replace("\n", "");
        };

        return name;
    };

    panic!("can't get the host, securely you are in a unsoported system")
}
