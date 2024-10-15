use regex::Regex;
use std::collections::HashMap;
use termion::color;
use unicode_width::UnicodeWidthStr;

pub fn make_columns(left: Vec<String>, right: Vec<String>) -> String {
    let max_left_length = left
        .iter()
        .map(|line| UnicodeWidthStr::width(strip_ansi_codes(line).as_str()))
        .max()
        .unwrap();

    let mut output = String::new();
    let total_lines = std::cmp::max(left.len(), right.len());
    let space = "".to_string();
    let padding = max_left_length + 4;
    for i in 0..total_lines {
        let left_line = left.get(i).unwrap_or(&space);
        let right_line = right.get(i).unwrap_or(&space);
        let padding_spaces =
            " ".repeat(padding - UnicodeWidthStr::width(strip_ansi_codes(left_line).as_str()));

        let columned_line = format!("{}{}{}\n", left_line, padding_spaces, right_line);
        output.push_str(&columned_line)
    }

    output
}

pub fn vectorize_string_file(text: &str) -> Vec<String> {
    text.split('\n').map(|item| item.to_string()).collect()
}
fn strip_ansi_codes(text: &str) -> String {
    let re = Regex::new(r"\x1b[\[\(][0-9;]*[A-Za-z~]").unwrap();
    re.replace_all(text, "").to_string()
}

pub fn parse_color(input: &str) -> String {
    let re = Regex::new(r"\$([0-9_]+)").unwrap();
    let mut vars: HashMap<&str, &str> = HashMap::new();
    vars.insert("1", color::Red.fg_str());
    vars.insert("2", color::Blue.fg_str());
    vars.insert("3", color::Green.fg_str());
    vars.insert("4", color::Cyan.fg_str());
    vars.insert("5", color::Magenta.fg_str());
    vars.insert("6", color::Black.fg_str());
    vars.insert("7", color::White.fg_str());

    let output = re.replace_all(input, |c: &regex::Captures| {
        let var = &c[1];
        match vars.get(var) {
            Some(v) => v,
            None => "$undefined_var",
        }
    });

    if output.contains("$undefined_var") {
        eprintln!("{}", input);
        std::process::exit(1)
    }

    output.to_string() + color::Reset.fg_str()
}
