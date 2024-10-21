use crate::config::ConfigFile;

// T: is the config for the
trait ModuleVar<T> {
    fn new() -> Self; // Creation method (Only define the name, for calculate the value method instead)
    fn value(&mut self, cfg: Option<T>);
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    todo!()
}

mod os;
