mod config;

fn main() {
    let config_opts = config::get_config();

    println!("Hello, your options are:");
    println!("{:#?}", config_opts)
}
