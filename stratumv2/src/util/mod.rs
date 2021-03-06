use crate::error::{Error, Result};
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

/// ByteParser is a custom iterator-like struct. It's used to extract segments
/// from a slice using by providing an offset to return the bytes from start
/// to step.
pub(crate) struct ByteParser<'a> {
    bytes: &'a [u8],
    start: usize,
}

impl<'a> ByteParser<'a> {
    pub(crate) fn new(bytes: &'a [u8], start: usize) -> ByteParser {
        ByteParser { bytes, start }
    }

    pub(crate) fn next_by(&mut self, step: usize) -> Result<&'a [u8]> {
        let offset = self.start + step;

        let b = self.bytes.get(self.start..offset);
        if b.is_none() {
            return Err(Error::ParseError("out of bounds error".into()));
        }

        self.start = offset;
        Ok(b.unwrap())
    }
}
