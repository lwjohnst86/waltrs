use dirs::home_dir;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

fn timestamp() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    now
}

fn clock_path() -> std::path::PathBuf {
    // This function returns the path to the clock file.
    // It uses the home directory and appends "Desktop/clock.json".
    // You can change this to any other path as needed.
    // For example, you might want to store it in a different directory.

    home_dir().unwrap().join("Desktop/timesheet.json")
}
