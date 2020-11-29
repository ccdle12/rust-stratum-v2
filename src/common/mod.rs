use crate::error::Result;
use std::io;

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
// TODO: Maybe just return bytes?
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Trait for converting an enum or type into it's byte representation according
/// to the Stratum V2 specification.
pub trait BitFlag {
    fn as_byte(&self) -> u8;
}

/// Trait for creating a serialized frame for networked messages. This trait
/// will build the correct frame for a specific message as well as serialize
/// the payload.
pub trait Framable {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Messages common to all Stratum V2 protocols.
mod messages;
pub use messages::{
    SetupConnection, SetupConnectionError, SetupConnectionErrorCodes, SetupConnectionSuccess,
};
