use std::collections::HashMap;
use std::fs;

use crate::config::file::GpuConfig;
use crate::error::LazyfetchError;

use super::ModuleVar;

static DEFAULT_PCI_IDS: &str = include_str!("../assets/pci.ids");

#[derive(Clone, Debug, Default)]
struct Vendor {
    name: String,
    devices: HashMap<String, String>,
}

impl Vendor {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: HashMap::new(),
        }
    }
}

pub struct GpuVar;

impl ModuleVar<GpuConfig> for GpuVar {
    fn name(self) -> String {
        String::from("gpu")
    }

    // NOTE:
    // i only use an integrated AMD gpu, so the names of other vendors can be wrong formatted because are not tested at all
    // If you have another gpu and see messy/wrong output please open an issue with an image of the output and your gpu name
    #[cfg(target_os = "linux")]
    fn value(self, cfg: Option<&GpuConfig>) -> Result<String, LazyfetchError> {
        let config = cfg.unwrap();
        let ids = parse_pci_ids(DEFAULT_PCI_IDS);
        let entries = fs::read_dir("/sys/class/drm")?;
        let mut gpus: Vec<(String, String)> = Vec::new();
        for entry in entries {
            let (name, path) = if let Ok(e) = entry {
                (e.file_name(), e.path())
            } else {
                continue;
            };

            let name = name.to_string_lossy();
            let device_path = path.join("device");

            if !name.starts_with("card")
                || !fs::exists(device_path.join("device"))?
                || !fs::exists(device_path.join("vendor"))?
            {
                continue;
            }

            let vendor_id = fs::read_to_string(device_path.join("vendor"))?;
            let device_id = fs::read_to_string(device_path.join("device"))?;

            let vendor = ids.get(vendor_id[2..].trim()).unwrap();
            let device = vendor.devices.get(device_id[2..].trim()).unwrap();

            gpus.push((vendor.name.clone(), device.to_string()));
        }

        let gpu = flat_gpu_name((&gpus[0].0, &gpus[0].1));
        if !config.show_brand.unwrap_or(false) {
            return Ok(gpu.1);
        }

        Ok(format!("{} {}", gpu.0, gpu.1))
    }
}

fn parse_pci_ids(id_list: &str) -> HashMap<String, Vendor> {
    let mut vendors: HashMap<String, Vendor> = HashMap::new();

    let mut vendor_id = "";
    for line in id_list.split('\n') {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if !line.starts_with('\t') {
            let vendor_data = line.split_once(' ').unwrap(); // 0: vendor id   1: vendor name
            vendor_id = vendor_data.0;
            vendors.insert(vendor_data.0.to_string(), Vendor::new(vendor_data.1));
        }

        let device_data = line.split_once(' ').unwrap();
        if let Some(vendor) = vendors.get_mut(&mut vendor_id.to_string()) {
            vendor.devices.insert(
                device_data.0.trim().to_string(),
                device_data.1.trim().to_string(),
            );
        }
    }

    vendors
}

fn flat_gpu_name(gpu: (&str, &str)) -> (String, String) {
    let flated_name = match gpu {
        g if g.0.contains("Advanced") => {
            let brand = match g {
                g if g.0.contains("[AMD/ATI]") => "AMD ATI",
                g if g.0.contains("[ATI]") => "ATI",
                g if g.0.contains("[AMD]") => "AMD",
                _ => "",
            };

            let name = g.1.replace(['[', ']'], "");
            (brand.trim().to_string(), name.trim().to_string())
        }
        g if g.1.contains("NVIDIA") => {
            (g.0.replace("Corporation", ""), g.1.replace(['[', ']'], ""))
        }
        g if g.1.contains("Intel") => {
            let name =
                g.1.replace("(R)", "")
                    .replace("Integrated Graphics Controller", "");

            (g.0.replace("Corporation", ""), name)
        }
        _ => (gpu.0.to_string(), gpu.1.to_string()),
    };

    flated_name
}
