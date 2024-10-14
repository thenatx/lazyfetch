use super::ModuleFn;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    UndefinedVar(String),
}

pub fn parse_module<'a>(
    input: &str,
    vars: &HashMap<&'a str, ModuleFn>,
) -> Result<String, ParseError> {
    let regex = Regex::new(r"\$\{([a-zA-Z0-9_]+)\}").unwrap();

    let output = regex
        .replace_all(input, |cap: &regex::Captures| {
            let var = &cap[1];
            match vars.get(var) {
                Some(func) => func(),
                None => "${undefined_var}".into(),
            }
        })
        .to_string();

    if output.contains("${undefined_var}") {
        let error_message = format!("Undefined var {}", input);
        return Err(ParseError::UndefinedVar(error_message));
    };

    Ok(output)
}

pub fn handle_parse_err(result: Result<String, ParseError>) -> String {
    match result {
        Ok(v) => v,
        Err(ParseError::UndefinedVar(err)) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
