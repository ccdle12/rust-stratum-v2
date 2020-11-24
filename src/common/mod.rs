use crate::error::Result;
use std::io;

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Trait for converting an enum or type into it's byte representation according
/// to the Stratum V2 specification.
pub trait BitFlag {
    fn as_byte(&self) -> u8;
}

/// Messages common to all Stratum V2 protocols.
mod messages;
pub use messages::SetupConnection;
