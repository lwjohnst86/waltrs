use dirs::home_dir;
use chrono::{DateTime, Utc};
use std::time::SystemTime;

/// Creates a timestamp of now.
/// 
/// This creates a timestamp of the current date and time in RFC 3339 format, with
/// UTC timezone as default. It is in the format `YYYY-MM-DDTHH:MM:SSZ`.
/// 
/// # Returns
/// 
/// - Returns a string representing the current timestamp in RFC 3339 format.
/// 
/// # Example
/// 
/// ```rust
/// use waltrs::common::timestamp;
/// let ts = timestamp();
/// println!("Current timestamp: {}", ts);
/// assert!(ts.starts_with("202"));
/// ```
pub fn timestamp() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let now = now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);

    now
}

pub fn timesheet_path() -> std::path::PathBuf {
    // This function returns the path to the clock file.
    // It uses the home directory and appends "Desktop/clock.json".
    // You can change this to any other path as needed.
    // For example, you might want to store it in a different directory.

    home_dir().unwrap().join(".waltrs/timesheet.json")
}

// Unit test for checking that the timestamp function outputs the expected format
#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    #[test]
    fn test_timestamp() {
        let ts = timestamp();
        // Check if the timestamp is in the expected format
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z").unwrap();
        assert!(re.is_match(&ts));
    }
  }
