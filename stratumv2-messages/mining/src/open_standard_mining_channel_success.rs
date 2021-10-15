use stratumv2_messages_sdk::impl_message;
use stratumv2_serde::types::{MessageType, B0_32, U256};

impl_message!(
    /// A message sent by the Server to the Client in response to a successful
    /// opening of a standard mining channel.
    OpenStandardMiningChannelSuccess,

    /// The request_id received in the [OpenStandardMiningChannel](struct.OpenStandardMiningChannel.html) message.
    /// This is returned to the Client so that they can pair the responses with the
    /// initial request.
    request_id u32,

    /// Assigned by the Server to uniquely identify the channel, the id is stable
    /// for the whole lifetime of the connection.
    channel_id u32,

    /// The initial target difficulty target for the mining channel.
    target U256,

    /// Bytes used as implicit first part of extranonce for the scenario when
    /// extended job is served by the upstream node for a set of standard
    /// channels that belong to the same group.
    extranonce_prefix B0_32,

    /// Group channel that the channel belongs to.
    group_channel_id u32
);

impl OpenStandardMiningChannelSuccess {
    pub fn new<T: Into<Vec<u8>>, U: Into<U256>>(
        request_id: u32,
        channel_id: u32,
        target: U,
        extranonce_prefix: T,
        group_channel_id: u32,
    ) -> Result<OpenStandardMiningChannelSuccess, stratumv2_serde::Error> {
        Ok(OpenStandardMiningChannelSuccess {
            request_id,
            channel_id,
            target: target.into(),
            extranonce_prefix: B0_32::new(extranonce_prefix)?,
            group_channel_id,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use stratumv2_messages_sdk::impl_message_tests;

    fn make_deserialized_open_standard_mining_channel_success() -> OpenStandardMiningChannelSuccess
    {
        OpenStandardMiningChannelSuccess::new(1u32, 2u32, [3u8; 32], [4u8; 4], 5u32).unwrap()
    }

    fn make_serialized_open_standard_mining_channel_success() -> Vec<u8> {
        vec![
            0x01, 0x00, 0x00, 0x00, // request_id,
            0x02, 0x00, 0x00, 0x00, // channel_id,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
            0x03, 0x03, 0x03, 0x03, // target
            0x04, 0x04, 0x04, 0x04, 0x04, // extranonce_prefix
            0x05, 0x00, 0x00, 0x00, // group_channel_id
        ]
    }

    impl_message_tests!(
        OpenStandardMiningChannelSuccess,
        make_serialized_open_standard_mining_channel_success,
        make_deserialized_open_standard_mining_channel_success
    );
}
