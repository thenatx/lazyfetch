// This function is used to handle the case when a function to get system info return a `None` value
pub fn handle_empty_var(module: Option<String>) -> String {
    match module {
        Some(v) => v,
        None => {
            eprintln!(
                "Error while getting this var value, seems like you're in a unsupported system"
            );
            std::process::exit(1);
        }
    }
}
