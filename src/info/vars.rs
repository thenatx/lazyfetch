use super::{cpu, disk, gpu, host, kernel, memory, os, shell, uptime, username, wm};
use super::{ModuleVar, ModuleVars};
use crate::config::file::ConfigFile;
use std::collections::HashMap;

macro_rules! insert_var {
    ($m:expr, $s:expr) => {
        let var_name = $s.name();
        $m.insert(var_name, Box::new(|| $s.value(None)))
    };

    ($m:expr, $s:expr, $c:expr) => {
        let var_name = $s.name();
        $m.insert(var_name, Box::new(|| $s.value($c)))
    };
}

pub fn init_vars<'a>(config: &'a ConfigFile) -> ModuleVars<'a> {
    let mut vars: ModuleVars = HashMap::new();
    insert_var!(vars, os::OsVar, config.os.as_ref());
    insert_var!(vars, username::UserNameVar);
    insert_var!(vars, host::HostNameVar);
    insert_var!(vars, host::HostVar);
    insert_var!(vars, kernel::Kernel);
    insert_var!(vars, wm::WindowManager, config.wm.as_ref());
    insert_var!(vars, memory::MemoryVar, config.memory.as_ref());
    insert_var!(vars, gpu::GpuVar, config.gpu.as_ref());
    insert_var!(vars, cpu::CpuVar, config.cpu.as_ref());
    insert_var!(vars, disk::DiskVar, config.disk.as_ref());
    insert_var!(vars, uptime::UptimeVar, config.uptime.as_ref());
    insert_var!(vars, shell::Shell);

    vars
}
