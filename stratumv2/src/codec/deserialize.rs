use crate::{codec::ByteParser, error::Result};

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
