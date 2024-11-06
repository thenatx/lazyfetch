use crate::config::GpuConfig;
use regex::Regex;
use std::process::Command;

use super::ModuleVar;

pub struct GpuVar;

impl ModuleVar<GpuConfig> for GpuVar {
    fn name(self) -> String {
        String::from("gpu")
    }

    fn value(self, cfg: Option<&GpuConfig>) -> String {
        if !cfg!(target_os = "linux") {
            eprintln!("Error: unsupported system");
            std::process::exit(1)
        }

        let config = cfg.unwrap();
        // NOTE: This implement for get the gpu info was taked from freshfetch source
        // I also plain to make a rust binding for the thing that uses lspci
        let lspci = {
            let lspci_cmd = Command::new("sh").arg("-c").arg("lspci -mm").output();
            match lspci_cmd {
                Ok(lscpi) => match String::from_utf8(lscpi.stdout) {
                    Ok(output) => output,
                    Err(err) => {
                        eprintln!("The output of the command contains invalid UTF8.\n{}", err);
                        panic!();
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                    panic!();
                }
            }
        };

        let mut gpus = {
            let mut to_return = Vec::new();
            let regex =
                Regex::new(r#"(?i)"(.*?(?:Display|3D|VGA).*?)" "(.*?\[.*?\])" "(?:.*?\[(.*?)\])""#)
                    .unwrap();
            let lspci_lines = lspci.split("\n").collect::<Vec<&str>>();
            for line in lspci_lines.iter() {
                let captures = regex.captures(line);
                if let Some(captures) = captures {
                    to_return.push((
                        String::from(captures.get(1).unwrap().as_str()),
                        String::from(captures.get(2).unwrap().as_str()),
                        String::from(captures.get(3).unwrap().as_str()),
                    ));
                }
            }

            to_return
        };

        if gpus.len() >= 2
            && gpus[0].1.to_lowercase().contains("intel")
            && gpus[1].1.to_lowercase().contains("intel")
        {
            gpus.pop();
        }

        let mut to_return = GpuStruct::new(String::new(), String::new());
        for gpu in gpus.iter_mut() {
            if gpu.1.to_lowercase().contains("advanced") {
                let mut brand = gpu.1.clone();
                let regex = Regex::new(r#".*?AMD.*?ATI.*?"#).unwrap();
                brand = String::from(regex.replace_all(&brand, "AMD ATI"));

                to_return = GpuStruct {
                    name: gpu.2.clone(),
                    brand: brand
                        .replace("[", "")
                        .replace("]", "")
                        .replace("OEM", "")
                        .replace("Advanced Micro Devices, Inc.", ""),
                }
            } else if gpu.1.to_lowercase().contains("nvidea") {
                to_return = GpuStruct::new(
                    gpu.2.clone(),
                    gpu.1.clone().replace("[", "").replace("]", ""),
                )
            } else if gpu.1.to_lowercase().contains("intel") {
                let mut brand = gpu.1.clone();
                brand = {
                    let regex = Regex::new(".*?Intel").unwrap();
                    String::from(regex.replace(&brand, "Intel"))
                };
                brand = brand.replace("(R)", "").replace("Corporation", "");
                brand = {
                    let regex = Regex::new(r#" \(.*?"#).unwrap();
                    String::from(regex.replace_all(&brand, ""))
                };
                brand = brand.replace("Integrated Graphics Controller", "");
                brand = {
                    let regex = Regex::new(r#".*?Xeon.*?"#).unwrap();
                    String::from(regex.replace(&brand, "Intel HD Graphics"))
                };
                brand = String::from(brand.trim());
                if brand.is_empty() {
                    brand = String::from("Intel HD Graphics");
                }
                to_return = GpuStruct::new(gpu.2.clone(), brand);
            }
        }

        if !config.show_brand.unwrap_or(false) {
            return to_return.name;
        }

        format!("{} {}", to_return.brand, to_return.name)
    }
}

struct GpuStruct {
    name: String,
    brand: String,
}

impl GpuStruct {
    fn new(name: String, brand: String) -> Self {
        Self { name, brand }
    }
}
