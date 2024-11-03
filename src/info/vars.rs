use super::{cpu, disk, gpu, host, memory, os, uptime, username};
use super::{ModuleVar, ModuleVars};
use crate::config::ConfigFile;
use std::collections::HashMap;

macro_rules! insert_var {
    ($m:expr, $s:expr) => {
        $m.insert($s.name, Box::new(|| $s.value(None)))
    };

    ($m:expr, $s:expr, $c:expr) => {
        $m.insert($s.name, Box::new(|| $s.value($c)))
    };
}

pub fn init_vars<'a>(config: &'a ConfigFile) -> ModuleVars<'a> {
    let mut vars: ModuleVars<'a> = HashMap::new();
    insert_var!(vars, os::OsVar::new(), config.os.as_ref());
    insert_var!(vars, username::UserNameVar::new());
    insert_var!(vars, host::HostNameVar::new());
    insert_var!(vars, host::HostVar::new());
    insert_var!(vars, memory::MemoryVar::new(), config.memory.as_ref());
    insert_var!(vars, gpu::GpuVar::new(), config.gpu.as_ref());
    insert_var!(vars, cpu::CpuVar::new(), config.cpu.as_ref());
    insert_var!(vars, disk::DiskVar::new(), config.disk.as_ref());
    insert_var!(vars, uptime::UptimeVar::new(), config.uptime.as_ref());

    vars
}
