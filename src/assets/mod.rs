use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Distro {
    name: &'static str,
    ascii: &'static str,
}

static LINUX_ASCII: &str = include_str!("./ascii/linux");
static DISTRO_LIST: &[Distro] = &[Distro {
    name: "nixos",
    ascii: include_str!("./ascii/nixos"),
}];

pub fn get_ascii(os_name: &str) -> &'static str {
    let distro = DISTRO_LIST
        .par_iter()
        .find_any(|distro| distro.name == os_name.to_lowercase());

    match distro {
        Some(d) => d.ascii,
        None => LINUX_ASCII,
    }
}
