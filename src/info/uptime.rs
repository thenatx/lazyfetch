use crate::{config::file::UptimeConfig, error::LazyfetchError};
use sysinfo::System;

use super::ModuleVar;

const SECONDS_IN_HOURS: f64 = 3600.0;
const SECONDS_IN_MINUTES: f64 = 60.0;
const SECONDS_IN_DAYS: f64 = 86400.0;

pub struct UptimeVar;

impl ModuleVar<UptimeConfig> for UptimeVar {
    fn name(self) -> String {
        String::from("uptime")
    }

    fn value(self, cfg: Option<&UptimeConfig>) -> Result<String, LazyfetchError> {
        let config = cfg.unwrap();
        let uptime_secs = System::uptime() as f64;

        let days = (uptime_secs / SECONDS_IN_DAYS) as u64;
        let hours = ((uptime_secs % SECONDS_IN_DAYS) / SECONDS_IN_HOURS) as u64;
        let minutes =
            (((uptime_secs % SECONDS_IN_DAYS) % SECONDS_IN_HOURS) / SECONDS_IN_MINUTES) as u64;

        if config.shorthand.unwrap_or(false) {
            if days == 0 && hours > 0 {
                return Ok(format!("{}h {}m", hours, minutes));
            } else if days == 0 && hours == 0 {
                return Ok(format!("{}m", minutes));
            }

            if hours == 0 && days > 0 {
                return Ok(format!("{}d {}m", days, minutes));
            }

            return Ok(format!("{}d {}h {}m", days, hours, minutes));
        }

        match (days, hours, minutes) {
            (1.., 0, 0) => Ok(format!("{} days", days)),
            (0, 1.., 0) => Ok(format!("{} hours", hours)),
            (0, 0, 1..) => Ok(format!("{} minutes", minutes)),
            (1.., 1.., 0) => Ok(format!("{} days, {} hours", days, hours)),
            (1.., 0, 1..) => Ok(format!("{} days, {} minutes", days, minutes)),
            (0, 1.., 1..) => Ok(format!("{} hours, {} minutes", hours, minutes)),
            _ => Ok(format!(
                "{} days, {} hours, {} minutes",
                days, hours, minutes
            )),
        }
    }
}
