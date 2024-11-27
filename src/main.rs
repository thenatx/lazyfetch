#![feature(never_type)]
mod assets;
mod colors;
mod config;
mod error;
mod info;
mod utils;

use error::LazyfetchError;

fn main() -> Result<(), LazyfetchError> {
    let (cli, config) = config::get_config();
    let system_info = info::get_info_lines(config)?;

    let ascii_lines = if let Some(distro) = cli.distro {
        utils::vectorize_string_file(assets::get_ascii(&distro))
    } else {
        let os_name = sysinfo::System::name().unwrap_or("linux".to_string());
        utils::vectorize_string_file(assets::get_ascii(&os_name))
    };

    let ascii: Vec<String> = ascii_lines
        .iter()
        .map(|line| Ok(utils::parse_color(line)?))
        .collect::<Result<Vec<_>, LazyfetchError>>()?;

    print!("{}", utils::make_columns(ascii, system_info)?);
    Ok(())
}
