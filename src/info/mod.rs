use crate::config::ConfigFile;

// T: is the config for the
trait ModuleVar<T> {
    fn new(config: Option<T>) -> Self; // Creation method that gets the config struct
    fn name(&self) -> String;
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    todo!()
}

mod os;
