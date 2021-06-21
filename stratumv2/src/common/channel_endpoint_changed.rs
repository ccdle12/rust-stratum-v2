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
    use crate::impl_message_tests;

    fn make_deserialized_channel_endpoint_changed() -> ChannelEndpointChanged {
        ChannelEndpointChanged::new(5u32).unwrap()
    }

    fn make_serialized_channel_endpoint_changed() -> Vec<u8> {
        return vec![0x05, 0x00, 0x00, 0x00];
    }

    impl_message_tests!(
        ChannelEndpointChanged,
        make_serialized_channel_endpoint_changed,
        make_deserialized_channel_endpoint_changed
    );
}
