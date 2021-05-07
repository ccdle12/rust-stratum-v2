use crate::error::Result;
use std::time::SystemTime;

/// Convert SystemTime to a Unix timestamp as a u32.
pub fn system_unix_time_to_u32(time: &SystemTime) -> Result<u32> {
    Ok(time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)?)
}

/// A helper function that returns the current unix time in seconds as a u32.
pub fn unix_u32_now() -> Result<u32> {
    system_unix_time_to_u32(&SystemTime::now())
}