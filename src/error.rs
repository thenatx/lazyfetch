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

pub fn invalid_var(m_content: &str) -> ! {
    eprintln!("Error: invalid var at the module \"{}\"", m_content);
    std::process::exit(1)
}
