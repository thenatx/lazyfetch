use super::ModuleVars;
use crate::error;
use regex::{Captures, Regex};

pub fn parse_vars(vars: &ModuleVars, content: &str) -> String {
    let re: Regex = Regex::new(r"\$\{([a-zA-Z]+)\}").unwrap();

    re.replace_all(content, |cap: &Captures| {
        let var = vars.get(&cap[1]);
        match var {
            Some(f) => f(),
            None => error::invalid_var(content, &cap[1]),
        }
    })
    .to_string()
}
