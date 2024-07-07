use sysinfo::System;

pub fn host_name() -> String {
    System::host_name().expect("Error getting the host name")
}
