// - check if timesheet file exists
// - check if timesheet file can be read/parsed
use std::fs::File;
use std::path::PathBuf;
use polars::prelude::*;

/// Reads the timesheet DataFrame from the timesheet JSON file.
/// 
/// # Arguments
/// 
/// - `path`: The path to the timesheet JSON file.
/// 
/// # Returns
/// 
/// - Returns a DataFrame containing the timesheet data.
/// 
/// # Panics
/// 
/// - If the file does not exist or cannot be opened, or if the DataFrame
/// cannot be read from the file.
pub fn read_timesheet(path: PathBuf) -> DataFrame {
    if !path.exists() {
        panic!("Timesheet file does not exist at {:?}", path);
    }

    let file: File = std::fs::File::open(path).expect("Unable to open the timesheet file.");

    let df: DataFrame = JsonReader::new(file)
        .with_json_format(JsonFormat::JsonLines)
        .finish()
        .expect("Unable to read DataFrame from file");

    df
}
