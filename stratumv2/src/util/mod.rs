use crate::Result;
use crate::{Frameable, Serializable};
use std::time::SystemTime;

mod channel_id;
pub use channel_id::new_channel_id;

/// Convert SystemTime to a Unix timestamp as a u32.
pub fn system_unix_time_to_u32(time: &SystemTime) -> Result<u32> {
    Ok(time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)?)
}

/// Helper utility function to frame a type that implements the Frameable trait
/// and returns the serialized result.
pub fn frame<T: Frameable>(val: T) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    val.frame(&mut buffer)?;

    Ok(buffer)
}

/// Helper utility function to serialize a type that implements the Serializable
/// trait and returns the serialized result.
pub fn serialize<T: Serializable>(val: T) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    val.serialize(&mut buffer)?;

    Ok(buffer)
}
