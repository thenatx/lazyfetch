use crate::{
    config::{ConfigFile, OsConfig},
    error,
};
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
        $m.insert($s.name, Box::new(|| $s.value(Some($c))))
    };
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    let separator = &config.output.separator.unwrap();
    let modules = &config.output.format;

    let mut output: Vec<String> = Vec::new();
    let vars = init_vars();

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
        println!("{}{separator}{}", parsed_key, parsed_content);
        output.push(format!("{}{separator}{}", parsed_key, parsed_content))
    }

    output
}

type ModuleVars<'a> = HashMap<&'a str, Box<dyn Fn() -> String>>;

fn init_vars<'a>() -> ModuleVars<'a> {
    let mut vars: ModuleVars = HashMap::new();
    insert_var!(vars, os::OsVar::new(), &OsConfig::default());
    insert_var!(vars, username::UserNameVar::new());
    insert_var!(vars, host::HostNameVar::new());
    insert_var!(vars, host::HostVar::new());
    insert_var!(vars, memory::MemoryVar::new());
    insert_var!(vars, gpu::GpuVar::new());

    vars
}

fn parse_vars<'a>(vars: &ModuleVars<'a>, content: &str) -> String {
    let re: Regex = Regex::new(r"\$\{([a-zA-Z]+)\}").unwrap();

    re.replace_all(content, |cap: &Captures| {
        let var = vars.get(&cap[1]);
        println!("{}", &cap[1]);
        match var {
            Some(f) => f(),
            None => error::invalid_var(&content, &cap[1]),
        }
    })
    .to_string()
}

mod gpu;
mod host;
mod memory;
mod os;
mod username;
