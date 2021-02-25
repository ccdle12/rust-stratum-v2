use crate::error::Error;
use crate::Result;
use std::time::SystemTime;

mod channel_id;
pub use channel_id::new_channel_id;

/// Convert SystemTime to a Unix timestamp as a u32.
pub fn system_unix_time_to_u32(time: &SystemTime) -> Result<u32> {
    Ok(time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)?)
}

/// Convert a byte array in little endian format to a u32.
pub(crate) fn le_bytes_to_u32(bytes: [u8; 4]) -> u32 {
    (bytes[3] as u32) << 24 | (bytes[2] as u32) << 16 | (bytes[1] as u32) << 8 | (bytes[0] as u32)
}

/// Convert a byte array in little endian format to a u16.
pub(crate) fn le_bytes_to_u16(bytes: [u8; 2]) -> u16 {
    (bytes[1] as u16) << 8 | (bytes[0] as u16)
}

/// Take a frame payload in bytes and extract the payload bytes.
pub fn deframe_payload(bytes: &[u8]) -> Result<Vec<u8>> {
    let length_bytes = *&bytes.get(3..6);
    if length_bytes.is_none() {
        return Err(Error::DeserializationError("missing payload length".into()));
    }

    let payload_length = length_bytes
        .unwrap()
        .into_iter()
        .map(|x| *x as u32)
        .fold(0, |accumulator, byte| (accumulator | byte)) as usize;

    bytes
        .get(6..6 + payload_length)
        .ok_or_else(|| Error::DeserializationError("failed to parse payload".into()))
        .map(|x| x.to_vec())
}
