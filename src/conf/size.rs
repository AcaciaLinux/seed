
use serde::{Deserialize, Serialize};

use super::part::*;

#[derive(Deserialize, Serialize, Debug)]
pub enum DataSize {
    B,
    KB,
    MB,
    GB,
    TB,
}
pub const DATA_SIZES: &'static [&'static str] = &["B", "KB", "MB", "GB", "TB"];

///	Matches a string of the data size value to the correct DataSize
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The DataSize, else the string that was not matched
pub fn match_data_size(value: &str) -> Result<DataSize, &str> {
    match value {
        "b" => Ok(DataSize::B),
        "kb" => Ok(DataSize::KB),
        "mb" => Ok(DataSize::MB),
        "gb" => Ok(DataSize::GB),
        "tb" => Ok(DataSize::TB),
        _ => Err(value),
    }
}

pub fn data_size_bytes(count: i64, unit: &DataSize) -> i64 {
    match unit {
        DataSize::B => count,
        DataSize::KB => count * 1024,
        DataSize::MB => count * 1024 * 1024,
        DataSize::GB => count * 1024 * 1024 * 1024,
        DataSize::TB => count * 1024 * 1024 * 1024 * 1024,
    }
}

#[derive(Serialize, Debug)]
pub enum PartSize {
    Size(u32, DataSize),
    PercentTotal(f32),
    PercentFree(f32),
    Total(PartTotal),
}
pub const PART_SIZES: &'static [&'static str] = &["%", "%%", "min/max..."];
