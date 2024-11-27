use std::fmt::{Debug, Formatter};
use thiserror::Error;

#[derive(Error)]
#[error(transparent)]
pub enum LazyfetchError {
    Io(#[from] std::io::Error),
    BadRegex(#[from] regex::Error),
    Deserialize(#[from] toml::de::Error),
    ParseInt(#[from] core::num::ParseIntError),
    EnveironmentVar(#[from] std::env::VarError),

    #[error("Use of a invalid variable '{0}' on the module '{1}'")]
    InvalidVar(String, String), // The 0 value is the undefined var used and the 1 var use the module where that var is tried to used

    #[error("{0}")]
    Custom(String),

    #[error("Something wrong happend")]
    Unknown,
}

impl Debug for LazyfetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LazyfetchError::Unknown
            | LazyfetchError::Custom(_)
            | LazyfetchError::InvalidVar(_, _) => {
                write!(f, "{}", self)
            }
            _ => write!(f, "{}: {:#}", self, self),
        }
    }
}
