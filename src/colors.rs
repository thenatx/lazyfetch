use regex::{Captures, Regex};
use termion::color;

use crate::error::LazyfetchError;

trait ColorVar {
    fn add_color(value: Option<&str>) -> Result<String, LazyfetchError>; // value is used for subvalues like hex codes
}

pub fn colorize_info(content: &str) -> Result<String, LazyfetchError> {
    Ok(parse_colors(content)? + termion::color::Reset.fg_str())
}

fn parse_colors(content: &str) -> Result<String, LazyfetchError> {
    let re = Regex::new(r"\$\{color:(#?[a-zA-Z-0-9]+)\}")?;
    let content = crate::utils::replace_regex_matches(&re, content, |cap: &Captures| {
        let m = &cap[1];
        let color = match m.to_lowercase().as_str() {
            "red" | "r" => color::Red.fg_str(),
            "green" | "g" => color::Green.fg_str(),
            "yellow" | "y" => color::Yellow.fg_str(),
            "blue" | "b" => color::Blue.fg_str(),
            "cyan" | "c" => color::Cyan.fg_str(),
            "magenta" | "m" => color::Magenta.fg_str(),
            "white" => color::White.fg_str(),
            "black" => color::Black.fg_str(),
            other => {
                if !is_hex_color(other) {
                    return Err(LazyfetchError::InvalidVar(
                        m.to_string(),
                        content.to_string(),
                    ));
                }

                &Hex::add_color(Some(other))
                    .map_err(|_| LazyfetchError::InvalidVar(m.to_string(), content.to_string()))?
            }
        };

        Ok(color.to_string())
    })?;

    Ok(content.to_string())
}

fn is_hex_color(hex: &str) -> bool {
    if !hex.contains('#') || !matches!(hex.len(), 4 | 7) {
        return false;
    }

    true
}

struct Hex;

impl ColorVar for Hex {
    fn add_color(value: Option<&str>) -> Result<String, LazyfetchError> {
        let hex_code = value.unwrap_or("#000000");

        let rgb = if hex_code.len() == 4 {
            Self::short_hex_to_rgb(hex_code[1..4].chars())?
        } else {
            Self::full_hex_to_rgb(hex_code)?
        };

        Ok(rgb.fg_string())
    }
}

impl Hex {
    #[allow(clippy::cast_possible_truncation)]
    fn short_hex_to_rgb(hex: std::str::Chars) -> Result<termion::color::Rgb, LazyfetchError> {
        let rgb: Vec<u8> = hex
            .map(|h| {
                if let Some(c) = h.to_digit(16) {
                    Ok(c as u8 * 0x11)
                } else {
                    Err(LazyfetchError::Unknown)
                }
            })
            .collect::<Result<Vec<_>, LazyfetchError>>()?;

        Ok(termion::color::Rgb(rgb[0], rgb[1], rgb[2]))
    }

    fn full_hex_to_rgb(hex: &str) -> Result<termion::color::Rgb, LazyfetchError> {
        fn parse_hex_double(bytes: &mut core::str::Bytes) -> Result<u8, LazyfetchError> {
            let group = [bytes.next().unwrap(), bytes.next().unwrap()];
            let s = core::str::from_utf8(&group).unwrap();

            Ok(u8::from_str_radix(s, 16)?)
        }

        let mut hex_bytes = hex.bytes();
        hex_bytes.next().unwrap();

        let r = parse_hex_double(&mut hex_bytes)?;
        let g = parse_hex_double(&mut hex_bytes)?;
        let b = parse_hex_double(&mut hex_bytes)?;

        Ok(termion::color::Rgb(r, g, b))
    }
}
