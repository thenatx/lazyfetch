use clap::Parser;
use cli::ClapOpts;
use file::ConfigFile;

pub fn get_config<'a>() -> (ClapOpts, ConfigFile<'a>) {
    let args = ClapOpts::parse();
    let config_file = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .join("lazyfetch")
        .join("config.toml");

    let config = if let Some(path) = &args.config {
        let path = path.to_owned();

        if path.exists() {
            ConfigFile::get_config_file(path)
        } else {
            ConfigFile::get_config_file(config_file)
        }
    } else {
        ConfigFile::get_config_file(config_file)
    };

    if let Ok(config) = config {
        (args, config)
    } else {
        (args, ConfigFile::default())
    }
}

mod cli;
pub mod file;
