use crate::error::Result;
use crate::frame::Frameable;
use crate::parse::{ByteParser, Deserializable, Serializable};
use crate::types::{new_channel_id, MessageType};
use std::io;

/// When a channel’s upstream or downstream endpoint changes and that channel had previously sent
/// messages with channel_msg bitset of unknown extension_type, the intermediate proxy MUST send a
/// ChannelEndpointChanged message. Upon receipt thereof, any extension state (including version
/// negotiation and the presence of support for a given extension) MUST be reset and
/// version/presence negotiation must begin again.
#[derive(Debug, Clone)]
pub struct ChannelEndpointChanged {
    /// The channel which has changed endpoint.
    channel_id: u32,
}

impl ChannelEndpointChanged {
    pub fn new() -> ChannelEndpointChanged {
        ChannelEndpointChanged {
            channel_id: new_channel_id(),
        }
    }
}

impl Serializable for ChannelEndpointChanged {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok(self.channel_id.serialize(writer)?)
    }
}

impl Deserializable for ChannelEndpointChanged {
    fn deserialize(parser: &mut ByteParser) -> Result<ChannelEndpointChanged> {
        let channel_id = u32::deserialize(parser)?;

        Ok(ChannelEndpointChanged {
            channel_id: channel_id,
        })
    }
}

impl Frameable for ChannelEndpointChanged {
    fn message_type() -> MessageType {
        MessageType::ChannelEndpointChanged
    }
}
