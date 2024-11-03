use crate::{config::ConfigFile, error};
use regex::{Captures, Regex};
use starbase_shell::ShellType;
use std::collections::HashMap;
use std::process::{Command, Stdio};

type ModuleVars<'a> = HashMap<&'a str, Box<dyn Fn() -> String + 'a>>;

// T: is the config for the
trait ModuleVar<T> {
    fn new() -> Self; // Creation method (Only define the name, for calculate the value use their method instead)
    fn value(self, cfg: Option<&T>) -> String;
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    let separator = &config.output.separator.as_ref().unwrap();
    let modules = &config.output.format;

    let mut output: Vec<String> = Vec::new();
    let vars = vars::init_vars(&config);

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
mod vars;
