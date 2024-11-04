use super::ModuleVar;

pub struct UserNameVar;

impl ModuleVar<!> for UserNameVar {
    fn name(self) -> String {
        String::from("username")
    }

    // In this case the cfg is `!` because there're no config options
    fn value(self, _cfg: Option<&!>) -> String {
        // TODO: Use other method to do this that works on all systems
        let user = std::env::var("USER");

        match user {
            Ok(u) => u,
            Err(_) => {
                eprintln!("Error, seems like you don't have the $USER enveironment variable defined in your system");
                std::process::exit(1)
            }
        }
    }
}
