use regex::{Captures, Regex};

use crate::error;

trait ColorVar {
    fn add_color(value: Option<&str>) -> String; // value is used for subvalues like hex codes
}

pub fn colorize_info(content: &str) -> String {
    parse_colors(content)
}

fn parse_colors(content: &str) -> String {
    let re = Regex::new(r"\$\{color:([a-zA-Z-0-9]+)\}").unwrap();

    re.replace_all(content, |cap: &Captures| {
        let m = &cap[1];

        match m {
            "red" => Red::add_color(None),
            "green" => Red::add_color(None),
            "blue" => Red::add_color(None),
            "yellow" => Red::add_color(None),
            "cyan" => Red::add_color(None),
            "magenta" => Red::add_color(None),
            "white" => Red::add_color(None),
            "black" => Red::add_color(None),
            other => {
                if !is_hex_color(&other) {
                    error::invalid_var(content, other)
                }

                Hex::add_color(Some(other))
            }
        }
    })
    .to_string()
}

fn is_hex_color(hex: &str) -> bool {
    if !hex.contains('#') {
        return false;
    }

    if !matches!(hex.len(), 4 | 7) {
        return false;
    }

    true
}

struct Red;

impl ColorVar for Red {
    fn add_color(_value: Option<&str>) -> String {
        String::from(termion::color::Red.fg_str())
    }
}

struct Hex;

impl ColorVar for Hex {
    fn add_color(value: Option<&str>) -> String {
        let hex_code = value.unwrap_or("#000000");
        let rgb = Self::full_hex_to_rgb(hex_code);

        rgb.fg_string()
    }
}

impl Hex {
    fn full_hex_to_rgb(hex: &str) -> termion::color::Rgb {
        let mut hex_bytes = hex.bytes();
        fn parse_hex_double(bytes: &mut core::str::Bytes) -> u8 {
            let group = [bytes.next().unwrap(), bytes.next().unwrap()];
            let s = core::str::from_utf8(&group).unwrap();

            u8::from_str_radix(s, 16).unwrap()
        }

        let r = parse_hex_double(&mut hex_bytes);
        let g = parse_hex_double(&mut hex_bytes);
        let b = parse_hex_double(&mut hex_bytes);

        termion::color::Rgb(r, g, b)
    }
}
