use crate::error::Result;
use crate::frame::Frameable;
use crate::parse::{ByteParser, Deserializable, Serializable};
use crate::types::{MessageType, B0_32, U256};
use std::io;

/// OpenStandardMiningChannelSuccess is a message sent by the Server to the Client
/// in response to a successful opening of a standard mining channel.
pub struct OpenStandardMiningChannelSuccess {
    /// The request_id received in the
    /// [OpenStandardMiningChannel](struct.OpenStandardMiningChannel.html) message.
    /// This is returned to the Client so that they can pair the responses with the
    /// initial request.
    request_id: u32,

    /// Assigned by the Server to uniquely identify the channel, the id is stable
    /// for the whole lifetime of the connection.
    channel_id: u32,

    /// The initial target difficulty target for the mining channel.
    target: U256,

    // TODO: I don't understand the purpose of the extranonce_prefix.
    extranonce_prefix: B0_32,

    /// Group channel that the channel belongs to.
    group_channel_id: u32,
}

impl OpenStandardMiningChannelSuccess {
    pub fn new<T: Into<Vec<u8>>>(
        request_id: u32,
        channel_id: u32,
        target: U256,
        extranonce_prefix: T,
        group_channel_id: u32,
    ) -> Result<OpenStandardMiningChannelSuccess> {
        Ok(OpenStandardMiningChannelSuccess {
            request_id,
            channel_id,
            target,
            extranonce_prefix: B0_32::new(extranonce_prefix)?,
            group_channel_id,
        })
    }
}

impl Serializable for OpenStandardMiningChannelSuccess {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let length = self.request_id.serialize(writer)?
            + self.channel_id.serialize(writer)?
            + self.target.serialize(writer)?
            + self.extranonce_prefix.serialize(writer)?
            + self.group_channel_id.serialize(writer)?;

        Ok(length)
    }
}

impl Deserializable for OpenStandardMiningChannelSuccess {
    fn deserialize(parser: &mut ByteParser) -> Result<Self> {
        OpenStandardMiningChannelSuccess::new(
            u32::deserialize(parser)?,
            u32::deserialize(parser)?,
            U256::deserialize(parser)?,
            B0_32::deserialize(parser)?,
            u32::deserialize(parser)?,
        )
    }
}

impl Frameable for OpenStandardMiningChannelSuccess {
    fn message_type() -> MessageType {
        MessageType::OpenStandardMiningChannelSuccess
    }
}
