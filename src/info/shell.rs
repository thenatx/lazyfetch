use starbase_shell::{ShellError, ShellType};

use super::ModuleVar;
use crate::error::LazyfetchError;

pub struct Shell;

impl ModuleVar<!> for Shell {
    fn name(self) -> String {
        String::from("shell")
    }

    // NOTE: This module may has to be re-wrtten by use an own implementation to detect the shell
    #[cfg(target_os = "linux")]
    fn value(self, _cfg: Option<&!>) -> Result<String, LazyfetchError> {
        let shell = match ShellType::try_detect() {
            Ok(v) => v.to_string(),
            Err(ShellError::UnknownShell { name }) => format!("Unknown shell: {}", name),
            Err(_) => "Can't detect the shell".to_string(),
        };

        Ok(shell)
    }
}
