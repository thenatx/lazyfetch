use crate::config::ConfigFile;

use srtemplate::SrTemplate;
use starbase_shell::ShellType;
use std::process::{Command, Stdio};
use termion::color;

const DEFAULT_SEPARATOR: &'static str = ": ";

pub fn parse(config: &ConfigFile) -> Vec<String> {
    let separator = match &config.output.separator {
        Some(separator) => separator,
        None => DEFAULT_SEPARATOR,
    };

    let key_template = set_key_vars();
    let content_template = set_content_vars();
    config
        .output
        .format
        .iter()
        .map(|module| {
            if module.content.len() < 1 {
                return format!("{}", replace_vars(&key_template, &module.key));
            }

            let content;
            if module.shell.unwrap_or_default() {
                content = exec_shell(&module.content);
            } else {
                content = replace_vars(&content_template, &module.content);
            }

            if module.key.len() < 1 {
                return format!("{}", content);
            }

            let key = replace_vars(&key_template, &module.key);
            format!("{}{}{}", key, &separator, content)
        })
        .collect()
}

fn replace_vars(context: &SrTemplate, content: &str) -> String {
    context.render(content).unwrap() + color::Reset.fg_str()
}

fn set_key_vars() -> SrTemplate<'static> {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context = set_color_vars(context);
    context.add_variable("username", &user::current());
    context.add_variable("hostname", &host::host_name());

    context
}

fn set_color_vars(context: SrTemplate<'static>) -> SrTemplate<'static> {
    context.add_variable("color:red", &color::Red.fg_str());
    context.add_variable("color:blue", &color::Blue.fg_str());
    context.add_variable("color:green", &color::Green.fg_str());
    context.add_variable("color:yellow", &color::Yellow.fg_str());
    context.add_variable("color:magenta", &color::Magenta.fg_str());
    context.add_variable("color:white", &color::White.fg_str());
    context.add_variable("color:black", &color::Black.fg_str());
    context
}

fn set_content_vars() -> SrTemplate<'static> {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context = set_color_vars(context);
    context.add_variable("username", &user::current());
    context.add_variable("hostname", &host::host_name());
    context.add_variable("host", &host::host());
    context.add_variable("uptime", &uptime::uptime(&crate::config::Uptime::default()));
    context.add_variable("os", &system::os(&crate::config::Os::default()));
    context.add_variable("cpu", &cpu::get_info(&crate::config::Cpu::default()));
    context.add_variable("gpu", &gpu::get_info(&crate::config::Gpu::default()));
    context.add_variable("disk", &disk::get_info(&crate::config::Disk::default()));
    context.add_variable(
        "memory",
        &memory::get_info(&crate::config::Memory::default()),
    );

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
        .expect("Failed to run the shell");

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
