use starbase_shell::ShellType;
use std::collections::HashMap;
use std::process::{Command, Stdio};

use crate::config::ConfigFile;
use crate::error::LazyfetchError;

type ModuleVars<'a> = HashMap<String, Box<dyn Fn() -> Result<String, LazyfetchError> + 'a>>;

// T: is the config struct for the var
trait ModuleVar<T> {
    fn name(self) -> String; // Creation method
    fn value(self, cfg: Option<&T>) -> Result<String, LazyfetchError>;
}

pub fn get_info_lines(config: ConfigFile) -> Result<Vec<String>, LazyfetchError> {
    let separator = &config.output.separator.clone().unwrap_or_default();
    let modules = &config.output.format;

    let mut output: Vec<String> = Vec::new();
    let vars = vars::init_vars(&config);

    for module in modules {
        if module.content.is_empty() {
            let parsed_key = parse::parse_vars(&vars, &module.key)?;
            output.push(crate::colors::colorize_info(&parsed_key)?);
            continue;
        }

        let parsed_content = if module.shell.unwrap_or(false) {
            exec_shell(&module.content)?
        } else {
            let content = crate::colors::colorize_info(&module.content)?;
            parse::parse_vars(&vars, &content)?
        };

        if module.key.is_empty() {
            output.push(parsed_content);
            continue;
        }

        let parsed_key = {
            let key = crate::colors::colorize_info(&module.key)?;
            parse::parse_vars(&vars, &key)?
        };

        output.push(format!("{}{separator}{}", parsed_key, parsed_content))
    }

    Ok(output)
}

fn exec_shell(input: &str) -> Result<String, LazyfetchError> {
    let shell = match ShellType::try_detect() {
        Ok(shell) => shell.to_string(),
        Err(_) => ShellType::Sh.to_string(),
    };

    let out = Command::new(shell)
        .arg("-c")
        .arg(input)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let mut output_string =
        String::from_utf8(out.stdout).expect("Error parsing the output to string");
    output_string.replace_range(output_string.len() - 1..output_string.len(), "");

    Ok(output_string)
}

mod cpu;
mod disk;
mod gpu;
mod host;
mod memory;
mod os;
mod parse;
mod uptime;
mod username;
mod vars;
