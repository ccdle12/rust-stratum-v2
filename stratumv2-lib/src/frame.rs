use crate::error::{Error, Result};
use crate::parse::{serialize, ByteParser, Deserializable, Serializable};
use crate::types::{MessageType, U24};
use std::io;

/// The CHANNEL_BIT_MASK is used to mask out the MSB to identify if a message
/// type has a channel id in a message frame.
const CHANNEL_BIT_MASK: u16 = 0x8000;

/// The EXTENSION_TYPE_MASK disables the MSB so the u16 representation of the
/// extension type in a message frame has the same value as the u15 representation.
const EXTENSION_TYPE_MASK: u16 = 0x7FFF;

/// Used to deserialize a received network frame. The payload would be further
/// deserialized according to the received MessageTypes.
pub struct Message {
    pub message_type: MessageType,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new<T: Into<Vec<u8>>>(message_type: MessageType, payload: T) -> Message {
        Message {
            message_type,
            payload: payload.into(),
        }
    }
}

impl Serializable for Message {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let message_type: u8 = self.message_type.msg_type();
        let mut extension_type: u16 = self.message_type.ext_type();
        // Enable the MSB to indicate if this message type has a channel id.
        if self.message_type.channel_bit() {
            extension_type |= CHANNEL_BIT_MASK;
        }

        let message_length = U24::new(self.payload.len() as u32)?;

        Ok([
            extension_type.serialize(writer)?,
            message_type.serialize(writer)?,
            message_length.serialize(writer)?,
            writer.write(self.payload.as_slice())?,
        ]
        .iter()
        .sum())
    }
}

impl Deserializable for Message {
    fn deserialize(parser: &mut ByteParser) -> Result<Message> {
        let mut extension_type = u16::deserialize(parser)?;
        let channel_bit: bool = (extension_type & CHANNEL_BIT_MASK) != 0;
        extension_type &= EXTENSION_TYPE_MASK;

        let message_type = MessageType::new(extension_type, u8::deserialize(parser)?)?;

        if message_type.channel_bit() != channel_bit {
            return Err(Error::UnexpectedChannelBit(channel_bit));
        }

        let message_length = U24::deserialize(parser)?;
        let payload = parser.next_by(message_length.0 as usize)?;

        // TODO(chpatton013): Make this operation zero-copy by taking the contents of the parser.
        Ok(Message::new(message_type, payload.to_vec()))
    }
}

/// Trait for wrapping and unwrapping messages with a network frame.
pub trait Frameable: Deserializable + Serializable {
    fn message_type() -> MessageType;
}

/// Utility function to create a network frame message according to a type
/// that implements the Frameable trait.
pub fn frame<T: Frameable>(payload: &T) -> Result<Message> {
    Ok(Message::new(T::message_type(), serialize(payload)?))
}

/// Utility function to convert a network frame message into a type that implements
/// the Frameable trait.
pub fn unframe<T: Frameable>(message: &Message) -> Result<T> {
    let expected_message_type = T::message_type();
    if expected_message_type != message.message_type {
        return Err(Error::UnexpectedMessageType(
            expected_message_type.ext_type(),
            expected_message_type.msg_type(),
        ));
    }

    let mut parser = ByteParser::new(message.payload.as_slice(), 0);

    T::deserialize(&mut parser)
}
