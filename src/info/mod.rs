use crate::config::ConfigFile;

trait ModuleVar<T> {
    fn new(config: Option<T>) -> Self;
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    todo!()
}

mod os;
