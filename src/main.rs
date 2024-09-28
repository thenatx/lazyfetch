mod assets;
mod config;
mod info;

fn main() {
    let (cli, config) = config::get_config();
    let info = info::parse(&config);

    let ascii = if cli.distro.is_some() {
        assets::get_ascii(cli.distro.unwrap().to_lowercase())
    } else {
        match sysinfo::System::name() {
            Some(name) => assets::get_ascii(name.to_lowercase()),
            None => assets::get_ascii("linux".to_string()),
        }
    };

    print!("{:>10}", ascii);
    for item in info {
        print!("{}", item);
    }
}
