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
    for distro in DISTRO_LIST {
      if distro.name == os_name.to_lowercase() {
        return distro.ascii
      }
    }

    LINUX_ASCII
}
