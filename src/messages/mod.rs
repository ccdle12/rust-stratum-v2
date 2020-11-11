use crate::error::Result;
use std::io;

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Messages common to all Stratum V2 protocols.
pub mod common;
