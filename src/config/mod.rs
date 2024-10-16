use clap::Parser;
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ClapOpts {
    // Config options
    #[arg(long, value_name = "PATH")]
    #[arg(help = "Path to the config file")]
    pub config: Option<PathBuf>,

    #[arg(long, exclusive = true)]
    #[arg(help = "If have to generate the default config, fails if you already have one")]
    pub gen_config: bool,

    #[arg(long, exclusive = true)]
    #[arg(help = "Force the creation of the config file, overwrite the previous config")]
    pub gen_config_force: bool,

    // Output options
    #[arg(long, value_name = "KEY")]
    #[arg(
        help = "Disable an specified info line from be in the output\n for example 'lazyfetch --disable memory' disable de lines with memory key"
    )]
    pub disable: Option<String>,

    #[arg(long, help = "Set the distro for the ascii art")]
    pub distro: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub separator: Option<String>,
    pub format: Vec<Module>,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            separator: Some(String::from(": ")),
            format: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub key: String,
    pub shell: Option<bool>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    pub ascii_art: Option<String>,
    pub stdout: Option<bool>,
}

impl Default for General {
    fn default() -> Self {
        Self {
            ascii_art: Some("".to_string()),
            stdout: Some(false),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Os {
    pub shorthand: Option<bool>,
    pub show_arch: Option<bool>,
}

impl Default for Os {
    fn default() -> Self {
        Self {
            shorthand: Some(false),
            show_arch: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uptime {
    pub shorthand: Option<bool>,
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            shorthand: Some(false),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    pub percent: Option<bool>,
    pub unit: Option<String>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            percent: Some(true),
            unit: Some("Mib".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cpu {
    pub speed_type: Option<String>,
    pub show_brand: Option<bool>,
    pub show_speed: Option<bool>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            speed_type: Some("bios_limit".to_string()),
            show_brand: Some(true),
            show_speed: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gpu {
    pub show_brand: Option<bool>,
}

impl Default for Gpu {
    fn default() -> Self {
        Self {
            show_brand: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    pub show_disk: Option<String>,
    pub subtitle: Option<String>,
    pub show_percent: Option<bool>,
}

impl Default for Disk {
    fn default() -> Self {
        Self {
            show_disk: Some("/".to_string()),
            subtitle: Some("none".to_string()),
            show_percent: Some(true),
        }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            output: Output::default(),
            general: Some(General {
                ascii_art: Some(String::from("")),
                stdout: Some(false),
            }),
            os: Some(Os::default()),
            uptime: Some(Uptime::default()),
            memory: Some(Memory::default()),
            cpu: Some(Cpu::default()),
            gpu: Some(Gpu::default()),
            disk: Some(Disk::default()),
        }
    }
}

pub fn get_config() -> (ClapOpts, ConfigFile) {
    static DEFAULT_CONFIG_FILE: &str = include_str!("./default.toml");
    let args = ClapOpts::parse();

    let config_path = if let Some(path) = &args.config {
        path.clone()
    } else {
        let config_path = directories::BaseDirs::new()
            .unwrap()
            .config_dir()
            .to_owned()
            .join("lazyfetch");

        let _ = std::fs::create_dir_all(config_path.clone());
        let config_file = config_path.join("config.toml");

        if !config_file.exists() {
            let _ = std::fs::write(config_file.clone(), DEFAULT_CONFIG_FILE);
        }

        config_file
    };

    if let Ok(config) = std::fs::read_to_string(config_path) {
        (args, toml::from_str::<ConfigFile>(&config).unwrap())
    } else {
        (args, ConfigFile::default())
    }
}
