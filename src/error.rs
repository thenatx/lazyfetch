use termion::color;

// This function is used to handle the case when a function to get system info return a `None` value
pub fn option_var_value<T>(value: Option<T>) -> T {
    match value {
        Some(v) => v,
        None => {
            eprintln!(
                "Error while try getting this var value, seems like you're in a unsupported system"
            );
            std::process::exit(1)
        }
    }
}

pub fn invalid_var(m_content: &str, var: &str) -> ! {
    let red_fg = color::Red.fg_str();
    let reset_fg = color::Reset.fg_str();

    let marked_error = format!("{}{}{}", red_fg, var, reset_fg);
    let module_with_marked_error = m_content.to_string().replace(var, &marked_error);

    eprintln!(
        "Error: invalid var used at the module \"{}\"",
        module_with_marked_error
    );
    std::process::exit(1)
}
