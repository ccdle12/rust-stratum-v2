/// Errors returned in the library.
pub mod error;

pub use crate::error::{Error, Result};
use std::io;

/// A custom iterator-like struct. It's used to extract segments
/// from a slice using by providing an offset to return the bytes from start
/// to step.
pub struct ByteParser<'a> {
    bytes: &'a [u8],
    start: usize,
}

impl<'a> ByteParser<'a> {
    pub fn new(bytes: &'a [u8], start: usize) -> ByteParser {
        ByteParser { bytes, start }
    }

    pub fn next_by(&mut self, step: usize) -> Result<&'a [u8]> {
        let offset = self.start + step;

        let b = self.bytes.get(self.start..offset);
        if b.is_none() {
            return Err(Error::ParseError("out of bounds error".into()));
        }

        self.start = offset;
        Ok(b.unwrap())
    }
}
/// Trait for deserializing bytes to most Stratum V2 messages.
pub trait Deserializable {
    fn deserialize(parser: &mut ByteParser) -> Result<Self>
    where
        Self: std::marker::Sized;
}

/// Helper utility function to deserialize a byte-stream into a type that
/// implements the Serializable trait and returns the deserialized result.
pub fn deserialize<T: Deserializable>(bytes: &[u8]) -> Result<T> {
    let mut parser = ByteParser::new(bytes, 0);
    T::deserialize(&mut parser)
}

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

/// Types used in all Stratum V2 Protocols.
pub mod types;
