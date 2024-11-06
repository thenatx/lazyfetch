use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ClapOpts {
    #[arg(long, value_name = "PATH")]
    #[arg(help = "Path to the config file")]
    pub config: Option<PathBuf>,

    #[arg(long, exclusive = true)]
    #[arg(help = "If have to generate the default config, fails if you already have one")]
    pub gen_config: bool,

    #[arg(long, exclusive = true)]
    #[arg(help = "Force the creation of the config file, overwrite the previous config")]
    pub gen_config_force: bool,

    #[arg(long, help = "Set the distro for the ascii art")]
    pub distro: Option<String>,
}
