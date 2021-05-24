use crate::error::Result;
use crate::impl_message;
use crate::types::MessageType;
use crate::types::{STR0_255, U256};

impl_message!(
    /// A message sent by the Client to the Server to open a mining channel that
    /// has additional capabilities such as difficulty aggregatation and custom
    /// search space splitting.
    OpenExtendedMiningChannel,

    /// A Client-specified unique identifier across all client connections.
    /// The request_id is not interpreted by the Server.
    request_id u32,

    /// A sequence of bytes that identifies the node to the Server, e.g.
    /// "braiintest.worker1".
    user_identity STR0_255,

    /// The expected [h/s] (hash rate/per second) of the
    /// device or the cumulative on the channel if multiple devices are connected
    /// downstream. Proxies MUST send 0.0f when there are no mining devices
    /// connected yet.
    nominal_hash_rate f32,

    /// The Maximum Target that can be acceptd by the connected device or
    /// multiple devices downstream. The Server MUST accept the maximum
    /// target or respond by sending a
    /// [OpenStandardMiningChannel.Error](struct.OpenStandardMiningChannelError.html)
    /// or [OpenExtendedMiningChannel.Error](struct.OpenExtendedMiningChannelError.html)
    max_target U256,

    /// The minimum size of extranonce space required by the Downstream node.
    min_extranonce_size u16
);

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::{frame, unframe, Message};
    use crate::parse::{deserialize, serialize};

    fn default_message() -> Result<OpenExtendedMiningChannel> {
        let message = OpenExtendedMiningChannel::new(
            1,
            "braiinstest.worker1".to_string(),
            12.3,
            U256([0u8; 32]),
            10,
        );
        assert!(message.is_ok());

        message
    }

    #[test]
    fn serialize_open_extended_mining_channel() {
        let expected = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x13, // length_user_identity
            0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x77, 0x6f,
            0x72, 0x6b, 0x65, 0x72, 0x31, // user_identity
            0xcd, 0xcc, 0x44, 0x41, // nominal_hash_rate
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // max_target
            0x0a, 0x00, // min_extranonce_size
        ];

        let serialized = serialize(&default_message().unwrap()).unwrap();
        assert_eq!(&serialized, &expected);
        assert!(deserialize::<OpenExtendedMiningChannel>(&serialized).is_ok());
    }

    #[test]
    fn frame_open_extended_mining() {
        let network_message = frame(&default_message().unwrap()).unwrap();
        assert_eq!(
            network_message.message_type,
            MessageType::OpenExtendedMiningChannel
        );

        let result = serialize(&network_message).unwrap();

        // Check the extension type is empty.
        assert_eq!(result[0..=1], [0u8; 2]);

        // Check that the correct byte for the message type was used.
        assert_eq!(result[2], network_message.message_type.msg_type());

        // Check the network message can be unframed.
        let deserialized = deserialize::<Message>(&result).unwrap();
        assert!(unframe::<OpenExtendedMiningChannel>(&deserialized).is_ok());
    }
}
