use crate::config::ConfigFile;

trait ModuleVar {
    fn new() -> Self;
}

pub fn get_info_lines(config: ConfigFile) -> Vec<String> {
    todo!()
}

mod os;
