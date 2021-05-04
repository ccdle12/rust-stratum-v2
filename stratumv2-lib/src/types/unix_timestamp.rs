use crate::error::Result;
use std::time::SystemTime;

/// Convert SystemTime to a Unix timestamp as a u32.
pub fn system_unix_time_to_u32(time: &SystemTime) -> Result<u32> {
    Ok(time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)?)
}

/// An helper macro to get the unix timestamp now in seconds as a wrapped u32
/// Result.
#[macro_export]
macro_rules! unix_u32_now {
    () => {
        system_unix_time_to_u32(&SystemTime::now())
    };
}
