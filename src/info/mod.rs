use super::config::Output;
use run_shell::cmd;

pub fn parse(config: Output) -> Vec<String> {
    let separator = config.separator.unwrap_or_default();
    return config
        .format
        .iter()
        .map(|module| {
            if module.shell.unwrap() == true {
                let key = module.key.clone();
                let content = cmd!(&module.content).stdout_utf8().unwrap();

                if key.len() <= 0 {
                    return format!("{}", content);
                }

                return format!("{}{}{}", key, separator, content);
            }

            format!("{}{}{}", module.key, separator, module.content)
        })
        .collect();
}
