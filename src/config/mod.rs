use clap::Parser;
use cli::ClapOpts;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConfigFile {
    pub output: Output,
    pub general: Option<General>,
    pub os: Option<Os>,
    pub uptime: Option<Uptime>,
    pub memory: Option<Memory>,
    pub cpu: Option<Cpu>,
    pub gpu: Option<Gpu>,
    pub disk: Option<Disk>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Output {
    pub separator: Option<String>,
    pub format: Vec<Module>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Module {
    pub key: String,
    pub shell: Option<bool>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct General {
    pub ascii_art: Option<String>,
    pub stdout: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Os {
    pub shorthand: Option<bool>,
    pub show_arch: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Uptime {
    pub shorthand: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Memory {
    pub percent: Option<bool>,
    pub unit: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Cpu {
    pub speed_type: Option<String>,
    pub show_brand: Option<bool>,
    pub show_speed: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Gpu {
    pub show_brand: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Disk {
    pub show_disk: Option<String>,
    pub subtitle: Option<String>,
    pub show_percent: Option<bool>,
}

impl ConfigFile {
    fn get_config_file(file_path: PathBuf) -> Result<Self, std::io::Error> {
        if !file_path.exists() {
            let _ = std::fs::create_dir_all(&file_path);
            let _ = std::fs::write(&file_path, DEFAULT_CONFIG_FILE);
        }

        let content = std::fs::read_to_string(file_path)?;
        Ok(toml::from_str(&content).unwrap())
    }
}

static DEFAULT_CONFIG_FILE: &str = include_str!("./default.toml");
pub fn get_config() -> (ClapOpts, ConfigFile) {
    let args = ClapOpts::parse();
    let config_path = directories::BaseDirs::new()
        .unwrap()
        .config_dir()
        .join("lazyfetch")
        .join("config.toml");

    let config_path = if let Some(path) = &args.config {
        let path = path.to_owned();

        if !path.exists() {
            ConfigFile::get_config_file(config_path)
        } else {
            ConfigFile::get_config_file(path)
        }
    } else {
        ConfigFile::get_config_file(config_path)
    };

    if let Ok(config) = config_path {
        (args, config)
    } else {
        (args, ConfigFile::default())
    }
}

mod cli;
