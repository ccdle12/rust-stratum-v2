use crate::error::{Error, Result};
use crate::parse::{serialize, ByteParser, Deserializable, Serializable};
use crate::types::{MessageType, U24};
use std::io;

/// The CHANNEL_BIT_MASK is used to mask out the MSB to identify if a message
/// type has a channel id in a message frame.
pub(crate) const CHANNEL_BIT_MASK: u16 = 0x8000;

/// The EXTENSION_TYPE_MASK disables the MSB so the u16 representation of the
/// extension type in a message frame has the same value as the u15 representation.
const EXTENSION_TYPE_MASK: u16 = 0x7FFF;

/// Used to deserialize a received network frame. The payload would be further
/// deserialized according to the received MessageTypes. Message can also be
/// used to store messages on an outgoing buffer before being processed
/// and sent over the wire.
#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_message;
    use crate::parse::{deserialize, serialize};

    impl_message!(TestMessage1, x u8);

    impl TestMessage1 {
        fn new(x: u8) -> Result<TestMessage1> {
            Ok(TestMessage1 { x })
        }
    }

    impl_message!(TestMessage2, x u8);

    impl TestMessage2 {
        fn new(x: u8) -> Result<TestMessage2> {
            Ok(TestMessage2 { x })
        }
    }

    #[test]
    fn frame_test_message() {
        let unframed = TestMessage1::new(5u8).unwrap();
        let framed = Message::new(MessageType::TestMessage1, vec![0x05]);
        assert_eq!(frame(&unframed).unwrap(), framed);
        assert_eq!(unframe::<TestMessage1>(&framed).unwrap(), unframed);
    }

    #[test]
    fn test_message_1_frame_serde() {
        let deserialized = Message::new(MessageType::TestMessage1, vec![0x05]);
        let serialized = vec![
            0x00, 0x00, // extension type & channel bit (MSB=0)
            0xfe, // message type
            0x01, 0x00, 0x00, // message length
            0x05, // message payload
        ];
        assert_eq!(serialize(&deserialized).unwrap(), serialized);
        assert_eq!(
            deserialize::<Message>(serialized.as_slice()).unwrap(),
            deserialized
        );
    }

    #[test]
    fn test_message_2_frame_serde() {
        let deserialized = Message::new(MessageType::TestMessage2, vec![0x05]);
        let serialized = vec![
            0x00, 0x80, // extension type & channel bit (MSB=1)
            0xff, // message type
            0x01, 0x00, 0x00, // message length
            0x05, // message payload
        ];
        assert_eq!(serialize(&deserialized).unwrap(), serialized);
        assert_eq!(
            deserialize::<Message>(serialized.as_slice()).unwrap(),
            deserialized
        );
    }
}
