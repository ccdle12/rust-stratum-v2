use crate::error::Result;
use crate::frame::Frameable;
use crate::parse::{ByteParser, Deserializable, Serializable};
use crate::types::{MessageType, STR0_255, U256};
use std::io;

/// OpenExtendedMiningChannel is a message sent by the Client to the Server
/// to open a mining channe that has additional capabilities such as
/// difficulty aggregate and custom search space splitting.
pub struct OpenExtendedMiningChannel {
    /// A Client-specified unique identifier across all client connections.
    /// The request_id is not interpreted by the Server.
    pub request_id: u32,

    /// A sequence of bytes that identifies the node to the Server, e.g.
    /// "braiintest.worker1".
    pub user_identity: STR0_255,

    /// The expected [h/s] (hash rate/per second) of the
    /// device or the cumulative on the channel if multiple devices are connected
    /// downstream. Proxies MUST send 0.0f when there are no mining devices
    /// connected yet.
    pub nominal_hash_rate: f32,

    /// The Maximum Target that can be acceptd by the connected device or
    /// multiple devices downstream. The Server MUST accept the maximum
    /// target or respond by sending a
    /// [OpenStandardMiningChannel.Error](struct.OpenStandardMiningChannelError.html)
    /// or [OpenExtendedMiningChannel.Error](struct.OpenExtendedMiningChannelError.html)
    pub max_target: U256,

    /// The minimum size of extranonce space required by the Downstream node.
    pub min_extranonce_size: u16,
}

impl OpenExtendedMiningChannel {
    pub fn new<T: Into<String>>(
        request_id: u32,
        user_identity: T,
        nominal_hash_rate: f32,
        max_target: U256,
        min_extranonce_size: u16,
    ) -> Result<OpenExtendedMiningChannel> {
        Ok(OpenExtendedMiningChannel {
            request_id,
            user_identity: STR0_255::new(user_identity)?,
            nominal_hash_rate,
            max_target,
            min_extranonce_size,
        })
    }
}

impl Serializable for OpenExtendedMiningChannel {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let length = self.request_id.serialize(writer)?
            + self.user_identity.serialize(writer)?
            + self.nominal_hash_rate.serialize(writer)?
            + self.max_target.serialize(writer)?
            + self.min_extranonce_size.serialize(writer)?;

        Ok(length)
    }
}

impl Deserializable for OpenExtendedMiningChannel {
    fn deserialize(parser: &mut ByteParser) -> Result<Self> {
        OpenExtendedMiningChannel::new(
            u32::deserialize(parser)?,
            STR0_255::deserialize(parser)?,
            f32::deserialize(parser)?,
            U256::deserialize(parser)?,
            u16::deserialize(parser)?,
        )
    }
}

impl Frameable for OpenExtendedMiningChannel {
    fn message_type() -> MessageType {
        MessageType::OpenExtendedMiningChannel
    }
}
