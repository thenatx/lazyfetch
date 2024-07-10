mod config;
mod info;

fn main() {
    let (_, config) = config::get_config();
    let info = info::parse(&config);

    for item in info {
        println!("{}", item);
    }
}
