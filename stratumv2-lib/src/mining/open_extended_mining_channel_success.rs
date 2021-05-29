use crate::error::Result;
use crate::impl_message;
use crate::types::{MessageType, B0_32, U256};

impl_message!(
    /// OpenExtendedMiningChannelSuccess is a message sent by the Server to the Client
    /// in response to a successful opening of a standard mining channel.
    OpenExtendedMiningChannelSuccess,

    /// The request_id received in the
    /// [OpenExtendedMiningChannel](struct.OpenExtendedMiningChannel.html) message.
    /// This is returned to the Client so that they can pair the responses with the
    /// initial request.
    request_id u32,

    /// Assigned by the Server to uniquely identify the channel, the id is stable
    /// for the whole lifetime of the connection.
    channel_id u32,

    /// The initial target difficulty target for the mining channel.
    target U256,

    // TODO: I don't understand the purpose of the extranonce size.
    extranonce_size u16,

    // TODO: I don't understand the purpose of the extranonce prefix.
    extranonce_prefix B0_32
);

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_message_tests;

    fn make_deserialized_open_extended_mining_channel_success() -> OpenExtendedMiningChannelSuccess
    {
        OpenExtendedMiningChannelSuccess::new(1u32, 2u32, U256([3u8; 32]), 4u16, [5u8; 4]).unwrap()
    }

    fn make_serialized_open_extended_mining_channel_success() -> Vec<u8> {
        return vec![
            0x01, 0x00, 0x00, 0x00, // request_id,
            0x02, 0x00, 0x00, 0x00, // channel_id,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, // target
            0x04, 0x00, // extranonce_size
            0x04, 0x05, 0x05, 0x05, 0x05, // extranonce_prefix
        ];
    }

    impl_message_tests!(
        OpenExtendedMiningChannelSuccess,
        make_serialized_open_extended_mining_channel_success,
        make_deserialized_open_extended_mining_channel_success
    );
}
