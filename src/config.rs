use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ClapOpts {
    // Config options
    #[arg(long, value_name = "PATH")]
    #[arg(help = "Path to the config file")]
    pub cofig_file: Option<PathBuf>,

    #[arg(long)]
    #[arg(help = "If have to generate the default config, fails if you already have one")]
    pub gen_config: bool,

    #[arg(long)]
    #[arg(help = "Force the creation of the config file, overwrite the previous config")]
    pub gen_config_force: bool,

    // Output options
    #[arg(long, help = "Disable an specified info line from be in the output")]
    pub disable: Option<String>,

    #[arg(long, help = "Set the distro for the ascii art")]
    pub distro: Option<String>,
}

pub fn get_config() -> ClapOpts {
    let cli_opts = ClapOpts::parse();

    cli_opts
}
