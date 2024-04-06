use super::error::IOError;
use std::fs;

pub fn read_to_string(file_path: &str) -> Result<String, IOError> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
