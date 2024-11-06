#![feature(never_type)]
mod assets;
mod colors;
mod config;
mod error;
mod info;
mod utils;

fn main() {
    let (cli, config) = config::get_config();
    let system_info = info::get_info_lines(config);

    let ascii_lines = if cli.distro.is_some() {
        let ascii = assets::get_ascii(cli.distro.unwrap().to_lowercase());
        utils::vectorize_string_file(ascii)
    } else {
        let ascii = match sysinfo::System::name() {
            Some(name) => assets::get_ascii(name.to_lowercase()),
            None => assets::get_ascii("linux".to_string()),
        };

        utils::vectorize_string_file(ascii)
    };

    let ascii: Vec<String> = ascii_lines
        .iter()
        .map(|line| utils::parse_color(line))
        .collect();

    print!("{}", utils::make_columns(ascii, system_info))
}
