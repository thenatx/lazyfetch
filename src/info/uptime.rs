use sysinfo::System;

use crate::config::Uptime;

const SECONDS_IN_HOURS: f64 = 3600.0;
const SECONDS_IN_MINUTES: f64 = 60.0;
const SECONDS_IN_DAYS: f64 = 86400.0;

pub fn uptime(config: &Uptime) -> String {
    let uptime_secs = System::uptime() as f64;

    let days = (uptime_secs / SECONDS_IN_DAYS) as u64;
    let hours = (uptime_secs % SECONDS_IN_DAYS) / SECONDS_IN_HOURS;
    let minutes = ((uptime_secs % SECONDS_IN_DAYS) % SECONDS_IN_HOURS) / SECONDS_IN_MINUTES;

    if config.shorthand.unwrap() {
        if days > 0 {
            return format!("{}d {}h {}m", days as u64, hours as u64, minutes as u64);
        }

        return format!("{}h {}m", hours, minutes);
    }

    if days > 0 {
        return format!("{} days {} hours {} minutes", days, hours, minutes);
    }

    format!("{} hours {} minutes", hours, minutes)
}
