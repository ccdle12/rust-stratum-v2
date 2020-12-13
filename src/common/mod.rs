use crate::error::Result;
use std::io;

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Trait for converting an enum or type into it's byte representation (u32)
/// according to the Stratum V2 specification.
pub trait BitFlag {
    fn as_bytes(&self) -> u32;
}

/// Trait for creating a serialized frame for networked messages. This trait
/// will build the correct frame for a specific message as well as serialize
/// the payload.
pub trait Framable {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Trait for identifying some struct/enum as part of a certain sub protocol.
pub trait ToProtocol {
    fn as_protocol(&self) -> Protocol;
}

/// Messages common to all Stratum V2 protocols.
mod messages;
pub use messages::{
    Protocol, SetupConnection, SetupConnectionError, SetupConnectionErrorCodes,
    SetupConnectionSuccess,
};
