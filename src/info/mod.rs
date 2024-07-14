use crate::config::ConfigFile;

use srtemplate::SrTemplate;
use starbase_shell::ShellType;
use std::process::{Command, Stdio};

const DEFAULT_SEPARATOR: &str = ": ";

pub fn parse(config: &ConfigFile) -> Vec<String> {
    let key_template = set_key_vars();
    let content_template = set_content_vars();
    let separator = match &config.output.separator {
        Some(separator) => separator,
        None => DEFAULT_SEPARATOR,
    };

    let mut to_return = Vec::new();
    for module in config.output.format.iter() {
        let mut content = module.content.clone();
        let key = replace_vars(&key_template, &module.key);

        if module.shell.unwrap_or_default() {
            content = exec_shell(&content);
        } else {
            content = replace_vars(&content_template, &content);
        }

        if key.len() == 0 {
            to_return.push(format!("{}", content));
            continue;
        }

        to_return.push(format!("{}{}{}", key, &separator, content));
    }

    to_return
}

fn replace_vars(context: &SrTemplate, content: &str) -> String {
    context.render(content).unwrap()
}

fn set_key_vars() -> SrTemplate<'static> {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context.add_variable("username", &user::current());
    context.add_variable("hostname", &host::host_name());

    context
}

fn set_content_vars() -> SrTemplate<'static> {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context.add_variable("username", &user::current());
    context.add_variable("hostname", &host::host_name());
    context.add_variable("host", &host::host());
    context.add_variable("uptime", &uptime::uptime(&crate::config::Uptime::default()));
    context.add_variable("os", &system::os(crate::config::Os::default()));
    context.add_variable("cpu", &cpu::get_info(&crate::config::Cpu::default()));
    context.add_variable("gpu", &gpu::get_info(&crate::config::Gpu::default()));
    context.add_variable(
        "memory",
        &memory::get_info(&crate::config::Memory::default()),
    );
    context.add_variable("disk", &disk::get_info(&crate::config::Disk::default()));

    context
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

mod cpu;
mod disk;
mod gpu;
mod host;
mod memory;
mod system;
mod uptime;
mod user;
