use super::ModuleVars;
use crate::error::LazyfetchError;
use regex::{Captures, Regex};

pub fn parse_vars(vars: &ModuleVars, content: &str) -> Result<String, LazyfetchError> {
    let re: Regex = Regex::new(r"\$\{([a-zA-Z]+)\}")?;

    let parsed_content = crate::utils::replace_regex_matches(&re, content, |cap: &Captures| {
        let var = vars.get(&cap[1]);
        match var {
            Some(f) => Ok(f()?),
            None => Err(LazyfetchError::InvalidVar(
                cap[1].to_string(),
                content.to_string(),
            )),
        }
    })?;
    Ok(parsed_content.to_string())
}
