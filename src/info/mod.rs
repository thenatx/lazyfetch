use crate::config::ConfigFile;
use starbase_shell::ShellType;
use std::process::{Command, Stdio};

type ModuleFn = Box<dyn Fn() -> String>;

const DEFAULT_SEPARATOR: &str = ": ";
pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    let separator = match &config.output.separator {
        Some(separator) => separator,
        None => DEFAULT_SEPARATOR,
    };

    let key_vars = vars::init_vars();
    let content_vars = vars::set_content_vars(config.clone());
    config
        .output
        .format
        .iter()
        .map(|module| {
            if module.content.is_empty() {
                return parse::handle_parse_err(parse::parse_module(&module.key, &key_vars));
            }

            let content = if module.shell.unwrap_or_default() {
                exec_shell(&module.content)
            } else {
                parse::handle_parse_err(parse::parse_module(&module.content, &content_vars))
            };

            if module.key.is_empty() {
                return content;
            }

            let key = parse::handle_parse_err(parse::parse_module(&module.key, &key_vars));
            format!("{}{}{}", key, &separator, content)
        })
        .collect()
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
mod parse;
mod system;
mod uptime;
mod user;
mod vars;
