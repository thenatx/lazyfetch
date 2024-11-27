use regex::Regex;
use std::collections::HashMap;
use termion::color;
use unicode_width::UnicodeWidthStr;

use crate::error::LazyfetchError;

pub fn make_columns(left: Vec<String>, right: Vec<String>) -> Result<String, LazyfetchError> {
    let max_left_length = left
        .iter()
        .map(|line| UnicodeWidthStr::width(strip_ansi_codes(line).unwrap().as_str()))
        .max()
        .unwrap();

    let mut output = String::new();
    let padding = max_left_length + 4;
    let empty_string = "".to_string();
    let total_lines = std::cmp::max(left.len(), right.len());
    for i in 0..total_lines {
        let left_line = left.get(i).unwrap_or(&empty_string);
        let right_line = right.get(i).unwrap_or(&empty_string);
        let padding_spaces =
            " ".repeat(padding - UnicodeWidthStr::width(strip_ansi_codes(left_line)?.as_str()));

        let columned_line = format!("{}{}{}\n", left_line, padding_spaces, right_line);
        output.push_str(&columned_line)
    }

    Ok(output)
}

pub fn vectorize_string_file(text: &str) -> Vec<String> {
    text.split('\n').map(|item| item.to_string()).collect()
}
fn strip_ansi_codes(text: &str) -> Result<String, LazyfetchError> {
    let re = Regex::new(r"\x1b[\[\(][0-9;]*[A-Za-z~]")?;
    Ok(re.replace_all(text, "").to_string())
}

pub fn parse_color(input: &str) -> Result<String, LazyfetchError> {
    let re = Regex::new(r"\$([0-9_]+)")?;
    let colors: HashMap<&str, &str> = HashMap::from([
        ("1", color::Red.fg_str()),
        ("2", color::Blue.fg_str()),
        ("3", color::Green.fg_str()),
        ("4", color::Cyan.fg_str()),
        ("5", color::Magenta.fg_str()),
        ("6", color::Black.fg_str()),
        ("7", color::White.fg_str()),
    ]);

    let output = replace_regex_matches(&re, input, |c: &regex::Captures| {
        let color = &c[1];
        match colors.get(color) {
            Some(v) => Ok(v.to_string()),
            None => {
                return Err(LazyfetchError::InvalidVar(
                    color.to_string(),
                    input.to_string(),
                ))
            }
        }
    })?;

    Ok(output + color::Reset.fg_str())
}

pub fn replace_regex_matches(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&regex::Captures) -> Result<String, LazyfetchError>,
) -> Result<String, LazyfetchError> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}
