use sysinfo::System;

use crate::config::Uptime;

const HOURS_IN_DAYS: u64 = 24;
const MINUTES_IN_HOURS: u64 = 60;
const SECONDS_IN_DAYS: u64 = 86400;

pub fn uptime(config: &Uptime) -> String {
    let uptime_secs = System::uptime();

    let days = uptime_secs / SECONDS_IN_DAYS;
    let hours = days / HOURS_IN_DAYS;
    let minutes = hours / MINUTES_IN_HOURS;

    if config.shorthand.unwrap() {
        return format!("{} d {} hrs {} mins", days, hours, minutes);
    }

    format!("{} days {} hours {} minutes", days, hours, minutes)
}
