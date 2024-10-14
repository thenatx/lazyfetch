use super::{cpu, disk, gpu, host, memory, system, uptime, user, ModuleFn};
use crate::config::ConfigFile;
use std::collections::HashMap;
use termion::color;

macro_rules! insert_var {
    ($vars:expr, $name:expr, $value:expr) => {
        $vars.insert($name, Box::new(move || $value.into()));
    };

    ($vars:expr, $name:expr, $closure:expr) => {
        $vars.insert($name, Box::new($closure));
    };
}

pub fn init_vars<'a>() -> HashMap<&'a str, ModuleFn> {
    let mut vars: HashMap<&'a str, ModuleFn> = HashMap::new();
    insert_var!(vars, "username", user::current());
    insert_var!(vars, "hostname", host::host_name());
    insert_var!(vars, "color:red", color::Red.fg_str());
    insert_var!(vars, "color:red", color::Red.fg_str());
    insert_var!(vars, "color:blue", color::Blue.fg_str());
    insert_var!(vars, "color:green", color::Green.fg_str());
    insert_var!(vars, "color:yellow", color::Yellow.fg_str());
    insert_var!(vars, "color:magenta", color::Magenta.fg_str());
    insert_var!(vars, "color:white", color::White.fg_str());
    insert_var!(vars, "color:black", color::Black.fg_str());
    vars
}

pub fn set_content_vars(config: ConfigFile) -> HashMap<&'static str, ModuleFn> {
    let mut vars = init_vars();
    insert_var!(vars, "host", host::host());
    insert_var!(
        vars,
        "uptime",
        uptime::uptime(&config.uptime.unwrap_or_default())
    );
    insert_var!(vars, "os", system::os(&config.os.unwrap()));
    insert_var!(vars, "cpu", cpu::get_info(&crate::config::Cpu::default()));
    insert_var!(vars, "gpu", gpu::get_info(&crate::config::Gpu::default()));
    insert_var!(
        vars,
        "disk",
        disk::get_info(&crate::config::Disk::default())
    );
    insert_var!(
        vars,
        "memory",
        memory::get_info(&crate::config::Memory::default())
    );

    vars
}
