use crate::error::Result;
use std::io;

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Helper utility function to serialize a type that implements the Serializable
/// trait and returns the serialized result.
pub fn serialize<T: Serializable>(val: &T) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    val.serialize(&mut buffer)?;

    Ok(buffer)
}
