//! A library implementation of the Stratum V2 Protocol.
//!
//! Stratum V2 sources:
//! - [Stratum V2 Overview](https://braiins.com/stratum-v2)
//! - [Stratum V2 Specification](https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit)
use crate::error::Result;
use std::io;

#[macro_use]
mod internal_macros;

/// Errors returned in the library.
mod error;

/// Types used in all Stratum V2 Protocols.
pub mod types;

/// Utility functions for all sub protocols.
pub mod util;

/// Common messages for all sub protocol.
pub mod common;

/// Mining is the main sub protocol of Stratum V2.
pub mod mining;

/// Job Negotiation is a sub protocol of Stratum V2.
pub mod job_negotiation;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Protocol is an enum representing each sub protocol of Stratum V2.
pub enum Protocol {
    /// Mining is the main and only required sub protocol in Stratum V2.
    Mining = 0,

    /// JobNegotiation is a protocol for intermediate nodes to broker
    /// the terms of a connection between downstream nodes and upstream nodes.
    JobNegotiation = 1,

    /// TemplateDistribution is a protocol for getting the next block from the
    /// Bitcoin RPC. This protocol is intented to replace getblocktemplate.
    TemplateDistribution = 2,

    /// JobDistribution is a protocol for passing newly-negotiated work from the
    /// Job Negotiator to proxies or mining devices. If miners aren't choosing
    /// their transaction sets, then jobs will be distributed from pools directly
    /// to proxies/mining devices.
    JobDistribution = 3,

    /// Unknown is catch-all variant. This should be used when attempting to
    /// convert another type into the Protocol enum but doesn't match any
    /// known variants.
    Unknown,
}

impl From<u8> for Protocol {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Protocol::Mining,
            1 => Protocol::JobNegotiation,
            2 => Protocol::TemplateDistribution,
            3 => Protocol::JobDistribution,
            _ => Protocol::Unknown,
        }
    }
}

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

/// Trait for deserializing bytes to most Stratum V2 messages.
pub trait Deserializable {
    fn deserialize(bytes: &[u8]) -> Result<Self>
    where
        Self: std::marker::Sized;
}

/// Trait for getting a types bit flag representation as a u32, according to the
/// Stratum V2 specification.
pub trait BitFlag {
    fn as_bit_flag(&self) -> u32;
    fn deserialize_flags(flags: u32) -> Vec<Self>
    where
        Self: std::marker::Sized;
}

/// Trait for creating a serialized frame for networked messages. This trait
/// will build the correct frame for a specific message as well as serialize
/// the payload.
pub trait Framable {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}
