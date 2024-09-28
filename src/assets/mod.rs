struct Distro {
    name: &'static str,
    ascii: &'static str,
    check_mode: Check,
}

// Match = The os distro name is equal to the their ascii name
// Contains = The os distro name contais their ascii name
enum Check {
    Match,
    Contains,
}

static DISTRO_LIST: &[Distro] = &[Distro {
    name: "nixos",
    ascii: include_str!("./nixos"),
    check_mode: Check::Match,
}];

pub fn get_ascii(os_name: String) -> &'static str {
    for distro in DISTRO_LIST {
        match distro.check_mode {
            Check::Match => {
                if distro.name == os_name {
                    return distro.ascii;
                }
            }
            Check::Contains => {
                if distro.name.contains(&os_name) {
                    return distro.ascii;
                }
            }
        }
    }

    return "";
}
