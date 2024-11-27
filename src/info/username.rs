use super::ModuleVar;
use crate::error::LazyfetchError;

pub struct UserNameVar;

impl ModuleVar<!> for UserNameVar {
    fn name(self) -> String {
        String::from("username")
    }

    // In this case the cfg is `!` because there're no config options
    fn value(self, _cfg: Option<&!>) -> Result<String, LazyfetchError> {
        // TODO: Use other method to do this that works on all systems
        Ok(std::env::var("USER")?)
    }
}
