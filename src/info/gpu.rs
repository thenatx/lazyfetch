use crate::config::GpuConfig;
use regex::Regex;
use std::process::Command;

use super::ModuleVar;
use crate::error::LazyfetchError;

pub struct GpuVar;

impl ModuleVar<GpuConfig> for GpuVar {
    fn name(self) -> String {
        String::from("gpu")
    }

    #[cfg(target_os = "linux")]
    #[allow(clippy::regex_creation_in_loops)]
    fn value(self, cfg: Option<&GpuConfig>) -> Result<String, LazyfetchError> {
        let config = cfg.unwrap();
        let lspci = {
            let lspci_cmd = Command::new("sh").arg("-c").arg("lspci -mm").output();
            String::from_utf8(lspci_cmd?.stdout).unwrap()
        };

        let mut gpus = {
            let mut to_return = Vec::new();
            let lspci_lines = lspci.split("\n").collect::<Vec<&str>>();
            let regex = Regex::new(
                r#"(?i)"(.*?(?:Display|3D|VGA).*?)" "(.*?\[.*?\])" "(?:.*?\[(.*?)\])""#,
            )?;
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
                let regex = Regex::new(r#".*?AMD.*?ATI.*?"#)?;
                brand = String::from(regex.replace_all(&brand, "AMD ATI"));

                to_return = GpuStruct::new(
                    gpu.2.clone(),
                    brand
                        .replace("[", "")
                        .replace("]", "")
                        .replace("OEM", "")
                        .replace("Advanced Micro Devices, Inc.", ""),
                );
                break;
            } else if gpu.1.to_lowercase().contains("nvidea") {
                to_return = GpuStruct::new(
                    gpu.2.clone(),
                    gpu.1.clone().replace("[", "").replace("]", ""),
                );
                break;
            } else if gpu.1.to_lowercase().contains("intel") {
                let mut brand = gpu.1.clone();
                brand = {
                    let regex = Regex::new(".*?Intel")?;
                    String::from(regex.replace(&brand, "Intel"))
                };
                brand = brand.replace("(R)", "").replace("Corporation", "");
                brand = {
                    let regex = Regex::new(r#" \(.*?"#)?;
                    String::from(regex.replace_all(&brand, ""))
                };
                brand = brand.replace("Integrated Graphics Controller", "");
                brand = {
                    let regex = Regex::new(r#".*?Xeon.*?"#)?;
                    String::from(regex.replace(&brand, "Intel HD Graphics"))
                };
                brand = String::from(brand.trim());
                if brand.is_empty() {
                    brand = String::from("Intel HD Graphics");
                }
                to_return = GpuStruct::new(gpu.2.clone(), brand);
                break;
            }
        }

        if !config.show_brand.unwrap_or(false) {
            return Ok(to_return.name);
        }

        Ok(format!("{} {}", to_return.brand, to_return.name))
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
