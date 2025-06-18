use polars::prelude::*;
use uuid::Uuid;

// - check if an entry is currently running and stop if there is
// - create new entry with `start` timestamp
// - ... with project
// - ... with project and tags
// - check if timesheet file exists
// - check if timesheet file can be read/parsed
// - check if id is unique
// - check if any `start` entries are later than `stop` entries
// - write to timesheet file

//fn start(project: String, tags: [str]) -> DataFrame {
//}


//fn start(project: &String, tags: [&String]) -> DataFrame {
fn start(project: &String) -> DataFrame {
    let id = Uuid::new_v4();
    df!(
        "id" => [id.to_string()],
        "start" => [common::timestamp()],
        "end" => [String::new()],
        "project" => [project],
    )
    // TODO: Replace with actual error handling.
    .unwrap()
}
