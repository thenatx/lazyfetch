use crate::{config::ConfigFile, error};
use regex::{Captures, Regex};
use starbase_shell::ShellType;
use std::collections::HashMap;
use std::process::{Command, Stdio};

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
        if module.content.is_empty() {
            let parsed_key = parse_vars(&vars, &module.key);
            output.push(parsed_key);
            continue;
        }

        let parsed_content = if module.shell.unwrap_or(false) {
            exec_shell(&module.content)
        } else {
            parse_vars(&vars, &module.content)
        };

        if module.key.is_empty() {
            output.push(parsed_content);
            continue;
        }

        let parsed_key = parse_vars(&vars, &module.key);
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

fn exec_shell(input: &str) -> String {
    let shell = match ShellType::try_detect() {
        Ok(shell) => shell.to_string(),
        Err(_) => ShellType::Sh.to_string(),
    };

    let out = Command::new(shell)
        .arg("-c")
        .arg(input)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .expect("Failed to run the shell");

    let mut output_string =
        String::from_utf8(out.stdout).expect("Error parsing the output to string");
    output_string.replace_range(output_string.len() - 1..output_string.len(), "");

    output_string
}

mod cpu;
mod disk;
mod gpu;
mod host;
mod memory;
mod os;
mod uptime;
mod username;
