use sysinfo::System;

use crate::config::Uptime;

const SECONDS_IN_HOURS: f64 = 3600.0;
const SECONDS_IN_MINUTES: f64 = 60.0;
const SECONDS_IN_DAYS: f64 = 86400.0;

pub fn uptime(config: &Uptime) -> String {
    let uptime_secs = System::uptime() as f64;

    let days = (uptime_secs / SECONDS_IN_DAYS) as u64;
    let hours = ((uptime_secs % SECONDS_IN_DAYS) / SECONDS_IN_HOURS) as u64;
    let minutes =
        (((uptime_secs % SECONDS_IN_DAYS) % SECONDS_IN_HOURS) / SECONDS_IN_MINUTES) as u64;

    if config.shorthand.unwrap() {
        if days == 0 && hours > 0 {
            return format!("{}h {}m", hours, minutes);
        } else if days == 0 && hours == 0 {
            return format!("{}m", minutes);
        }

        if hours == 0 && days > 0 {
            return format!("{}d {}m", days, minutes);
        }

        return format!("{}d {}h {}m", days, hours, minutes);
    }

    if days == 0 && hours == 0 {
        return format!("{} minutes", minutes);
    }

    if days == 0 && hours > 0 {
        return format!("{} hours, {} minutes", hours, minutes);
    }

    if hours == 0 && days > 0 {
        return format!("{} days, {} minutes", days, minutes);
    }

    format!("{} days, {} hours, {} minutes", days, hours, minutes)
}
