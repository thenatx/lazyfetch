use crate::config::{Module, Output};

use regex::Regex;
use starbase_shell::ShellType;
use sysinfo::System;

use std::fs::read_to_string;
use std::process::{Command, Stdio};

pub fn parse(config: Output) -> Vec<String> {
    let separator = config.separator.unwrap_or_default();

    return config
        .format
        .iter()
        .map(|module| parse_content(module, &separator))
        .collect();
}

fn parse_content(module: &Module, separator: &str) -> String {
    let mut content = module.content.clone();
    let key = replace_vars(module.key.clone());

    if module.shell.unwrap_or_default() {
        content = exec_shell(&content);
    } else {
        content = replace_vars(content);
    }

    if key.len() == 0 {
        return format!("{}", content.to_string());
    }

    format!("{}{}{}", key, separator, content)
}

fn replace_vars(content: String) -> String {
    let mut new_content = content.clone();
    let regex = Regex::new(r"\$\{(?:[^{}]|(?:\{[^{}]*\}))*\}").unwrap();
    let _: Vec<_> = regex
        .captures_iter(&content)
        .map(|m| {
            let matched_str = m.get(0).unwrap().as_str();

            // TODO: End the rest modules
            let replace_matches = match &matched_str[2..matched_str.len() - 1] {
                "username" => user::current(),
                "host" => String::new(),
                "os" => String::new(),
                "uptime" => String::new(),
                "cpu" => String::new(),
                "gpu" => String::new(),
                "disk" => String::new(),
                "memory" => String::new(),
                other => {
                    eprintln!("Error: the {} module not exists, check that is correcly written and exists", other);
                    std::process::exit(1)
                }
            };

            new_content = new_content.replace(
                matched_str,
                &replace_matches
            );
        })
        .collect();
    new_content
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

mod user;
