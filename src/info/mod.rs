use crate::config::ConfigFile;

use srtemplate::SrTemplate;
use starbase_shell::ShellType;
use std::process::{Command, Stdio};

const DEFAULT_SEPARATOR: &str = ": ";

pub fn parse(config: &ConfigFile) -> Vec<String> {
    let separator = config
        .output
        .separator
        .clone()
        .unwrap_or(DEFAULT_SEPARATOR.to_string());

    config
        .output
        .format
        .iter()
        .map(|module| {
            let mut content = module.content.clone();
            let key = replace_vars(&module.key.clone());

            if module.shell.unwrap_or_default() {
                content = exec_shell(&content);
            } else {
                content = replace_vars(&content);
            }

            if key.len() == 0 {
                return format!("{}", content.to_string());
            }

            format!("{}{}{}", key, separator, content)
        })
        .collect()
}

fn replace_vars(content: &str) -> String {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context.add_variable("username", &user::current());
    context.add_variable("hostname", &host::host_name());
    context.add_variable("host", &host::host());
    context.add_variable(
        "uptime",
        &uptime::uptime(&crate::config::Uptime {
            shorthand: Some(true),
        }),
    );
    context.add_variable(
        "os",
        &system::os(crate::config::Os {
            shorthand: Some(true),
            show_arch: Some(false),
        }),
    );

    context.render(content).unwrap()
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
        .expect("Failed to wait the command");

    let mut output_string =
        String::from_utf8(out.stdout).expect("Error parsing the output to string");
    let out_len = output_string.len();
    output_string.replace_range(out_len - 1..out_len, "");

    output_string
}

mod host;
mod system;
mod uptime;
mod user;
