use stratumv2_codec::Frameable;
use stratumv2_messages_sdk::impl_message;
use stratumv2_serde::{types::MessageType, ByteParser, Deserializable, Serializable};

impl_message!(
    /// When a channel’s upstream or downstream endpoint changes and that channel had previously sent
    /// messages with channel_msg bitset of unknown extension_type, the intermediate proxy MUST send a
    /// ChannelEndpointChanged message. Upon receipt thereof, any extension state (including version
    /// negotiation and the presence of support for a given extension) MUST be reset and
    /// version/presence negotiation must begin again.
    ChannelEndpointChanged,

    /// The channel which has changed endpoint.
    channel_id u32
);

impl ChannelEndpointChanged {
    pub fn new(channel_id: u32) -> Result<ChannelEndpointChanged, stratumv2_serde::Error> {
        Ok(ChannelEndpointChanged { channel_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stratumv2_messages_sdk::impl_message_tests;

    fn make_deserialized_channel_endpoint_changed() -> ChannelEndpointChanged {
        ChannelEndpointChanged::new(5u32).unwrap()
    }

    fn make_serialized_channel_endpoint_changed() -> Vec<u8> {
        vec![0x05, 0x00, 0x00, 0x00]
    }

    impl_message_tests!(
        ChannelEndpointChanged,
        make_serialized_channel_endpoint_changed,
        make_deserialized_channel_endpoint_changed
    );
}
