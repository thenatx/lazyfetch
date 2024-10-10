mod assets;
mod config;
mod info;
mod utils;

use srtemplate::SrTemplate;
use termion::color;

fn main() {
    let (cli, config) = config::get_config();
    let system_info = info::parse(&config);

    let ascii_lines = if cli.distro.is_some() {
        let ascii = assets::get_ascii(cli.distro.unwrap().to_lowercase());

        utils::vectorize_string(ascii)
    } else {
        let ascii = match sysinfo::System::name() {
            Some(name) => assets::get_ascii(name.to_lowercase()),
            None => assets::get_ascii("linux".to_string()),
        };

        utils::vectorize_string(ascii)
    };

    let ascii: Vec<String> = ascii_lines.iter().map(|line| parse_color(line)).collect();

    println!("{}", utils::make_columns(ascii, system_info))
}

fn parse_color(text: &str) -> String {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context.add_variable("1", &color::Red.fg_str());
    context.add_variable("2", &color::Blue.fg_str());
    context.add_variable("3", &color::Green.fg_str());
    context.add_variable("4", &color::Yellow.fg_str());
    context.add_variable("5", &color::Magenta.fg_str());
    context.add_variable("6", &color::White.fg_str());
    context.add_variable("0", &color::Black.fg_str());

    let colored_text = context.render(text).unwrap() + &color::Reset.fg_str();

    colored_text
}
