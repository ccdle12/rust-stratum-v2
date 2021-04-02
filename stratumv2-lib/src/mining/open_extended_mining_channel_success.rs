use crate::error::Result;
use crate::frame::Frameable;
use crate::parse::{ByteParser, Deserializable, Serializable};
use crate::types::{MessageType, B0_32, U256};
use std::io;

/// OpenExtendedMiningChannelSuccess is a message sent by the Server to the Client
/// in response to a successful opening of a standard mining channel.
pub struct OpenExtendedMiningChannelSuccess {
    /// The request_id received in the
    /// [OpenExtendedMiningChannel](struct.OpenExtendedMiningChannel.html) message.
    /// This is returned to the Client so that they can pair the responses with the
    /// initial request.
    request_id: u32,

    /// Assigned by the Server to uniquely identify the channel, the id is stable
    /// for the whole lifetime of the connection.
    channel_id: u32,

    /// The initial target difficulty target for the mining channel.
    target: U256,

    // TODO: I don't understand the purpose of the extranonce size.
    extranonce_size: u16,

    // TODO: I don't understand the purpose of the extranonce prefix.
    extranonce_prefix: B0_32,
}

impl OpenExtendedMiningChannelSuccess {
    pub fn new<T: Into<Vec<u8>>>(
        request_id: u32,
        channel_id: u32,
        target: U256,
        extranonce_size: u16,
        extranonce_prefix: T,
    ) -> Result<OpenExtendedMiningChannelSuccess> {
        Ok(OpenExtendedMiningChannelSuccess {
            request_id,
            channel_id,
            target,
            extranonce_size,
            extranonce_prefix: B0_32::new(extranonce_prefix)?,
        })
    }
}

impl Serializable for OpenExtendedMiningChannelSuccess {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let length = self.request_id.serialize(writer)?
            + self.channel_id.serialize(writer)?
            + self.target.serialize(writer)?
            + self.extranonce_size.serialize(writer)?
            + self.extranonce_prefix.serialize(writer)?;

        Ok(length)
    }
}

impl Deserializable for OpenExtendedMiningChannelSuccess {
    fn deserialize(parser: &mut ByteParser) -> Result<Self> {
        OpenExtendedMiningChannelSuccess::new(
            u32::deserialize(parser)?,
            u32::deserialize(parser)?,
            U256::deserialize(parser)?,
            u16::deserialize(parser)?,
            B0_32::deserialize(parser)?,
        )
    }
}

impl Frameable for OpenExtendedMiningChannelSuccess {
    fn message_type() -> MessageType {
        MessageType::OpenExtendedMiningChannelSuccess
    }
}
