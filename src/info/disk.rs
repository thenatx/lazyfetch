use crate::config::DiskConfig;

use super::ModuleVar;

const DEFAULT_DISK: &str = "/";
const BYTES_IN_GIGABYTES: u64 = 1000000000;

// TODO: refactor this function and implement "subtitle" config option
pub struct DiskVar<'a> {
    pub name: &'a str,
}

impl<'a> ModuleVar<DiskConfig> for DiskVar<'a> {
    fn new() -> Self {
        Self { name: "disk" }
    }

    fn value(self, cfg: Option<&DiskConfig>) -> String {
        let config = cfg.unwrap();
        let mut show_disk: &str = &config.show_disk.clone().unwrap_or(DEFAULT_DISK.to_string());

        if show_disk.is_empty() {
            show_disk = DEFAULT_DISK;
        }

        let disks = sysinfo::Disks::new_with_refreshed_list();

        let mut disk_info: DiskStruct = DiskStruct::new("".to_string(), 0, 0);
        for disk in &disks {
            let mount_point = disk.mount_point().to_str().unwrap();
            if show_disk != mount_point {
                break;
            }

            disk_info = DiskStruct::new(
                mount_point.to_string(),
                disk.total_space() / BYTES_IN_GIGABYTES,
                disk.available_space() / BYTES_IN_GIGABYTES,
            )
        }

        if disk_info.mount_point.is_empty() {
            eprintln!("Error: the mount point is empty");
            panic!()
        }

        let used_space = disk_info.total_space - disk_info.aviable_space;

        if config.show_percent.unwrap_or(true) {
            let percent = (used_space as f64 / disk_info.total_space as f64) * 100.0;
            return format!(
                "{}G / {}G ({}%)",
                used_space, disk_info.total_space, percent as u64
            );
        }

        format!("{}G / {}G", used_space, disk_info.total_space)
    }
}

struct DiskStruct {
    mount_point: String,
    total_space: u64,
    aviable_space: u64,
}

impl DiskStruct {
    fn new(mount_point: String, total_space: u64, aviable_space: u64) -> Self {
        Self {
            mount_point,
            total_space,
            aviable_space,
        }
    }
}
