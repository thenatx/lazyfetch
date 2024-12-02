use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf};

use crate::error::LazyfetchError;

static DEFAULT_CONFIG_FILE: &str = include_str!("./default.toml");

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ConfigFile<'a> {
    pub output: Output,

    #[serde(rename = "general")]
    pub general: Option<GeneralConfig>,

    #[serde(rename = "os")]
    pub os: Option<OsConfig>,

    #[serde(rename = "uptime")]
    pub uptime: Option<UptimeConfig>,

    #[serde(rename = "memory")]
    pub memory: Option<MemoryConfig<'a>>,

    #[serde(rename = "cpu")]
    pub cpu: Option<CpuConfig>,

    #[serde(rename = "gpu")]
    pub gpu: Option<GpuConfig>,

    #[serde(rename = "disk")]
    pub disk: Option<DiskConfig>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Output {
    pub separator: Option<String>,
    pub format: Vec<Module>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Module {
    pub key: String,
    pub shell: Option<bool>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct GeneralConfig {
    pub ascii_art: Option<String>,
    pub stdout: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct OsConfig {
    pub shorthand: Option<bool>,
    pub show_arch: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct UptimeConfig {
    pub shorthand: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct MemoryConfig<'a> {
    pub percent: Option<bool>,
    pub unit: Option<Cow<'a, str>>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CpuConfig {
    pub speed_type: Option<String>,
    pub show_brand: Option<bool>,
    pub show_speed: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub struct GpuConfig {
    pub show_brand: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DiskConfig {
    pub show_disk: Option<String>,
    pub subtitle: Option<String>,
    pub show_percent: Option<bool>,
}

impl ConfigFile<'_> {
    pub fn get_config_file(file_path: PathBuf) -> Result<Self, LazyfetchError> {
        if !file_path.exists() {
            let _ = std::fs::create_dir_all(&file_path);
            let _ = std::fs::write(&file_path, DEFAULT_CONFIG_FILE);
        }

        let content = std::fs::read_to_string(file_path)?;
        Ok(toml::from_str(&content)?)
    }
}
