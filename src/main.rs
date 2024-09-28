mod assets;
mod config;
mod info;

use srtemplate::SrTemplate;
use termion::color;

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

    print!("{:>10}", colorize_ascii(ascii));
    for item in info {
        print!("{}", item);
    }
}

fn colorize_ascii(ascii: &str) -> String {
    let mut context = SrTemplate::default();
    context.set_delimiter("${", "}");
    context.add_variable("red", &color::Red.fg_str());
    context.add_variable("blue", &color::Blue.fg_str());
    context.add_variable("green", &color::Green.fg_str());
    context.add_variable("yellow", &color::Yellow.fg_str());
    context.add_variable("magenta", &color::Magenta.fg_str());
    context.add_variable("white", &color::White.fg_str());
    context.add_variable("black", &color::Black.fg_str());

    context.render(ascii).unwrap()
}
