use crate::{config::ConfigFile, error};
use regex::{Captures, Regex};
use std::collections::HashMap;

// T: is the config for the
trait ModuleVar<T> {
    fn new() -> Self; // Creation method (Only define the name, for calculate the value use their method instead)
    fn value(self, cfg: Option<&T>) -> String;
}

macro_rules! insert_var {
    ($m:expr, $s:expr) => {
        $m.insert($s.name, Box::new(|| $s.value(None)))
    };

    ($m:expr, $s:expr, $c:expr) => {
        $m.insert($s.name, Box::new(|| $s.value($c)))
    };
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    let separator = &config.output.separator.as_ref().unwrap();
    let modules = &config.output.format;

    let mut output: Vec<String> = Vec::new();
    let vars = init_vars(&config);

    for module in modules {
        if module.key.is_empty() {
            let parsed_content = parse_vars(&vars, &module.content);
            output.push(parsed_content);
            continue;
        }

        if module.content.is_empty() {
            let parsed_key = parse_vars(&vars, &module.key);
            output.push(parsed_key);
            continue;
        }

        let parsed_key = parse_vars(&vars, &module.key);
        let parsed_content = parse_vars(&vars, &module.content);
        output.push(format!("{}{separator}{}", parsed_key, parsed_content))
    }

    output
}

type ModuleVars<'a> = HashMap<&'a str, Box<dyn Fn() -> String + 'a>>;

fn init_vars<'a>(config: &'a ConfigFile) -> ModuleVars<'a> {
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

fn parse_vars<'a>(vars: &'a ModuleVars, content: &str) -> String {
    let re: Regex = Regex::new(r"\$\{([a-zA-Z]+)\}").unwrap();

    re.replace_all(content, |cap: &Captures| {
        let var = vars.get(&cap[1]);
        match var {
            Some(f) => f(),
            None => error::invalid_var(&content, &cap[1]),
        }
    })
    .to_string()
}

mod cpu;
mod disk;
mod gpu;
mod host;
mod memory;
mod os;
mod uptime;
mod username;
