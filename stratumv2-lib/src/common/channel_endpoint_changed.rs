use crate::{
    error::Result,
    frame::Frameable,
    impl_message,
    parse::{ByteParser, Deserializable, Serializable},
    types::MessageType,
};
use std::io;

impl_message!(
    /// When a channel’s upstream or downstream endpoint changes and that channel had previously sent
    /// messages with channel_msg bitset of unknown extension_type, the intermediate proxy MUST send a
    /// ChannelEndpointChanged message. Upon receipt thereof, any extension state (including version
    /// negotiation and the presence of support for a given extension) MUST be reset and
    /// version/presence negotiation must begin again.
    ChannelEndpointChanged,
    MessageType::ChannelEndpointChanged,

    /// The channel which has changed endpoint.
    channel_id u32
);

impl ChannelEndpointChanged {
    pub fn new(channel_id: u32) -> Result<ChannelEndpointChanged> {
        Ok(ChannelEndpointChanged { channel_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{frame, unframe, Message};
    use crate::parse::{deserialize, serialize};

    #[test]
    fn init_channel_endpoint_changed() {
        let channel_endpoint_changed = ChannelEndpointChanged::new(0).unwrap();

        let serialized = serialize(&channel_endpoint_changed).unwrap();
        assert!(deserialize::<ChannelEndpointChanged>(&serialized).is_ok());
    }

    #[test]
    fn frame_open_extended_mining() {
        let network_message = frame(&ChannelEndpointChanged::new(0).unwrap()).unwrap();
        assert_eq!(
            network_message.message_type,
            MessageType::ChannelEndpointChanged
        );

        let result = serialize(&network_message).unwrap();

        // Check the extension type is NOT empty and the MSB is set.
        // NOTE: the MSB is serialized according to U16 so in serialized form
        // it will appear as the second byte.
        assert_eq!(result[0..=1], [0, 128]);

        // Check that the correct byte for the message type was used.
        assert_eq!(result[2], network_message.message_type.msg_type());

        // Check the network message can be unframed.
        let deserialized = deserialize::<Message>(&result).unwrap();
        assert!(unframe::<ChannelEndpointChanged>(&deserialized).is_ok());
    }
}
