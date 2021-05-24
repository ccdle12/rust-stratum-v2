use crate::error::Result;
use crate::impl_message;
use crate::types::{MessageType, B0_32, U256};

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

    // TODO: I don't understand the purpose of the extranonce_prefix.
    extranonce_prefix B0_32,

    /// Group channel that the channel belongs to.
    group_channel_id u32
);

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::{frame, unframe, Message};
    use crate::parse::{deserialize, serialize};
    use crate::types::new_channel_id;

    #[test]
    fn frame_open_standard_mining_success() {
        let extranonce_prefix = [0x00, 0x00];
        let message = OpenStandardMiningChannelSuccess::new(
            1,
            new_channel_id(),
            U256([0u8; 32]),
            extranonce_prefix,
            1,
        )
        .unwrap();

        let network_message = frame(&message).unwrap();
        let result = serialize(&network_message).unwrap();

        // Check the extension type is empty.
        assert_eq!(result[0..=1], [0u8; 2]);

        // Check that the correct byte for the message type was used.
        assert_eq!(result[2], network_message.message_type.msg_type());

        // Check the network message can be unframed.
        let deserialized = deserialize::<Message>(&result).unwrap();
        assert!(unframe::<OpenStandardMiningChannelSuccess>(&deserialized).is_ok());
    }
}
