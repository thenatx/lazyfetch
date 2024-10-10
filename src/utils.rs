use regex::Regex;
use unicode_width::UnicodeWidthStr;

pub fn make_columns(left: Vec<String>, right: Vec<String>) -> String {
    let max_left_lenght = left
        .iter()
        .map(|line| UnicodeWidthStr::width(strip_ansi_codes(line).as_str()))
        .max()
        .unwrap_or(0);

    let mut output = String::new();
    let padding = max_left_lenght + 4;
    let total_lines = std::cmp::max(left.len(), right.len());
    let spaces = " ".repeat(max_left_lenght);
    for i in 0..total_lines {
        let left_line = left.get(i).unwrap_or(&spaces);
        let right_line = right.get(i).unwrap_or(&spaces);

        output = format!("{} {:<padding$}", left_line, right_line,)
    }

    output
}

pub fn vectorize_string(text: &str) -> Vec<String> {
    let mut text_vector: Vec<String> = Vec::new();
    for i in text.split('\n') {
        text_vector.push(String::from(i));
    }

    text_vector
}

fn strip_ansi_codes(text: &str) -> String {
    let re = Regex::new(r"\x1b[\[\(][0-9;]*[A-Za-z~]").unwrap();
    re.replace_all(text, "").to_string()
}
