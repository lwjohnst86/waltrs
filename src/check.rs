use polars::prelude::*;

// - check if id is unique
// - check if any `start` entries are later than `stop` entries
// - check if an entry is currently running and stop if there is

pub fn check_unique_id(df: &DataFrame) -> Result<&DataFrame, PolarsError> {
    // Check if the ID already exists in the DataFrame
    let id_series = df.column("id").expect("Column 'id' not found");
    let check_answer = id_series.unique();
    match id_series.is_unique() {
        true => Ok(df),
        false => Err(PolarsError::ComputeError(
            "ID is not unique".to_string(),
        )),
    }
}
