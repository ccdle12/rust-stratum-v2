use crate::error::Result;
use std::io;

/// Types used in all Stratum V2 Protocols.
pub mod types;

/// Messages common to all Stratum V2 protocols.
pub mod messages;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
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
}

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
