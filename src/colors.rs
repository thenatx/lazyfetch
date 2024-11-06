use regex::{Captures, Regex};
use termion::color;

use crate::error;

trait ColorVar {
    fn add_color(value: Option<&str>) -> String; // value is used for subvalues like hex codes
}

pub fn colorize_info(content: &str) -> String {
    parse_colors(content) + termion::color::Reset.fg_str()
}

fn parse_colors(content: &str) -> String {
    let re = Regex::new(r"\$\{color:(#?[a-zA-Z-0-9]+)\}").unwrap();

    return re
        .replace_all(content, |cap: &Captures| {
            let m = &cap[1];

            match m.to_lowercase().as_str() {
                "red" | "r" => color::Red.fg_str().to_string(),
                "green" | "g" => color::Green.fg_str().to_string(),
                "blue" | "b" => color::Blue.fg_str().to_string(),
                "yellow" | "y" => color::Yellow.fg_str().to_string(),
                "cyan" | "c" => color::Cyan.fg_str().to_string(),
                "magenta" | "m" => color::Magenta.fg_str().to_string(),
                "white" => color::White.fg_str().to_string(),
                "black" => color::Black.fg_str().to_string(),
                other => {
                    if !is_hex_color(other) {
                        error::invalid_var(content, other)
                    }

                    Hex::add_color(Some(other))
                }
            }
        })
        .to_string();
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

struct Hex;

impl ColorVar for Hex {
    fn add_color(value: Option<&str>) -> String {
        let hex_code = value.unwrap_or("#000000");

        let rgb = if hex_code.len() == 4 {
            Self::short_hex_to_rgb(hex_code[1..4].chars())
        } else {
            Self::full_hex_to_rgb(hex_code)
        };

        rgb.fg_string()
    }
}

impl Hex {
    fn short_hex_to_rgb(hex: std::str::Chars) -> termion::color::Rgb {
        let rgb: Vec<u8> = hex
            .map(|h| match h {
                '0' => 0x00,
                '1' => 0x11,
                '2' => 0x22,
                '3' => 0x33,
                '4' => 0x44,
                '5' => 0x55,
                '6' => 0x66,
                '7' => 0x77,
                '8' => 0x88,
                '9' => 0x99,
                'a' | 'A' => 0xAA,
                'b' | 'B' => 0xBB,
                'c' | 'C' => 0xCC,
                'd' | 'D' => 0xDD,
                'e' | 'E' => 0xEE,
                'f' | 'F' => 0xFF,
                _ => panic!(),
            })
            .collect();

        termion::color::Rgb(rgb[0], rgb[1], rgb[2])
    }

    fn full_hex_to_rgb(hex: &str) -> termion::color::Rgb {
        let mut hex_bytes = hex.bytes();
        hex_bytes.next().unwrap();

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
