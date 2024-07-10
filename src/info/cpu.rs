use crate::config::Cpu;

use sysinfo::System;

pub fn get_info(config: &Cpu) -> String {
    let mut sys = System::new();
    sys.refresh_cpu();

    sys.cpus().iter().map(|cpu| println!("{:#?}", cpu));

    format!("")
}
