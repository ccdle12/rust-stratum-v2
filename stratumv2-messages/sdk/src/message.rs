pub mod message_macro_prelude {
    pub use std::io;
    pub use stratumv2_codec::Frameable;
    pub use stratumv2_serde::{types::MessageType, ByteParser, Deserializable, Serializable};
}

/// A macro to a StratumV2 message struct and it's required trait implmentations.
#[macro_export]
macro_rules! impl_message {
    (
      $(#[$doc_comment:meta])*
      $struct_name:ident,
      $($(#[$field_comment:meta])*
      $field: ident $field_type:ident),*
    ) => {
    #[allow(unused_imports)]
    use stratumv2_messages_sdk::message::message_macro_prelude::*;

    $(#[$doc_comment])*
    #[derive(Debug, Clone, PartialEq)]
    pub struct $struct_name {
      $(
        $(#[$field_comment])*
        pub $field: $field_type
      ),*
    }

    impl Serializable for $struct_name {
      fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize, stratumv2_serde::Error> {
        Ok([
          $(self.$field.serialize(writer)?,)*
        ]
        .iter()
        .sum())
      }
    }

    impl Deserializable for $struct_name {
      fn deserialize(parser: &mut ByteParser) -> Result<Self, stratumv2_serde::Error> {
        $struct_name::new(
          $($field_type::deserialize(parser)?,)*
        )
      }
    }

    impl Frameable for $struct_name {
      fn message_type() -> MessageType {
        MessageType::$struct_name
      }
    }
  };
}

pub mod message_tests_macro_prelude {
    pub use stratumv2_codec::{frame, unframe, Message};
    pub use stratumv2_serde::{deserialize, serialize};
}

/// A utility macro to test a StratumV2 message.
#[macro_export]
macro_rules! impl_message_tests {
    ($struct_name:ident, $make_serialized:ident, $make_deserialized:ident) => {
        #[allow(unused_imports)]
        use stratumv2_messages_sdk::message::message_tests_macro_prelude::*;

        #[test]
        fn message_serde_identity() {
            // Verify that "good" messages and message payloads (as defined by the
            // $make_deserialized and $make_serialized marco expression arguments) ser/de as an
            // identity function. This provides us with confidence that all fields in the message
            // encode correctly, and in the correct order.
            let deserialized = $make_deserialized();
            let serialized = $make_serialized();
            assert_eq!(serialize(&deserialized).unwrap(), serialized);
            assert_eq!(
                deserialize::<$struct_name>(serialized.as_slice()).unwrap(),
                deserialized
            );
        }

        #[test]
        fn message_type_tautology() {
            // Verify that this message's MessageType enum variant is the same as its struct name.
            assert_eq!($struct_name::message_type(), MessageType::$struct_name);
        }

        #[test]
        fn message_frame_identity() {
            // Verify that "good" messages and message payloads (un)frame as an identity function.
            let deserialized = $make_deserialized();
            let serialized = $make_serialized();
            let message = Message::new(MessageType::$struct_name, serialized);
            assert_eq!(frame(&deserialized).unwrap(), message);
            assert_eq!(unframe::<$struct_name>(&message).unwrap(), deserialized);
        }

        #[test]
        fn message_frame_serde() {
            // Verify that message frames for this type of message ser/de correctly.
            let serialized_frame = make_serialized_frame();
            let deserialized_frame = Message::new($struct_name::message_type(), $make_serialized());

            assert_eq!(serialize(&deserialized_frame).unwrap(), serialized_frame);
            assert_eq!(
                deserialize::<Message>(&serialized_frame).unwrap(),
                deserialized_frame
            );
        }

        fn make_serialized_frame() -> Vec<u8> {
            let serialized_message = $make_serialized();

            let mut extension_type = $struct_name::message_type().ext_type();
            if $struct_name::message_type().channel_bit() {
                extension_type |= stratumv2_codec::CHANNEL_BIT_MASK;
            }
            let message_type = $struct_name::message_type().msg_type();
            let message_length = serialized_message.len();

            let mut serialized_frame = vec![];
            serialized_frame.extend(extension_type.to_le_bytes().iter());
            serialized_frame.extend(message_type.to_le_bytes().iter());
            serialized_frame.extend(message_length.to_le_bytes()[..3].iter());
            serialized_frame.extend(serialized_message);

            serialized_frame
        }
    };
}
