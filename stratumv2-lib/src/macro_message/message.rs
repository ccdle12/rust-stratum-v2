pub mod macro_prelude {
    pub use crate::error::Result;
    pub use crate::frame::Frameable;
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::MessageType;
    pub use std::io;
}

/// Internal macro to build all the common requirements for a Stratum-v2 message.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_message {
    (
      $(#[$doc_comment:meta])*
      $struct_name:ident,
      $($(#[$field_comment:meta])*
      $field: ident $field_type:ident),*
    ) => {
    #[allow(unused_imports)]
    use crate::macro_message::message::macro_prelude::*;

    $(#[$doc_comment])*
    #[derive(Debug, Clone, PartialEq)]
    pub struct $struct_name {
      $(
        $(#[$field_comment])*
        pub $field: $field_type
      ),*
    }

    impl Serializable for $struct_name {
      fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok([
          $(self.$field.serialize(writer)?,)*
        ]
        .iter()
        .sum())
      }
    }

    impl Deserializable for $struct_name {
      fn deserialize(parser: &mut ByteParser) -> Result<Self> {
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

#[cfg(test)]
pub mod test_macro_prelude {
    pub use crate::frame::{frame, unframe, Frameable, Message};
    pub use crate::parse::{deserialize, serialize};
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_message_tests {
    ($struct_name:ident, $make_serialized:ident, $make_deserialized:ident) => {
        #[allow(unused_imports)]
        use crate::macro_message::message::test_macro_prelude::*;

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
                extension_type |= 0x8000;
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

#[cfg(test)]
mod test_message_tests {
    use crate::types::{B0_16M, B0_255, B0_31, B0_32, B0_64K, STR0_255, STR0_32, U24, U256};

    // A test message with all the primitives that comprise the real message types.
    //
    // This lets us make the assertion that other message implementations don't need to worry about
    // testing that these primitives "work" in ser/de operations.
    //
    // If a specific message type uses any fields outside of the types used here, it will be
    // responsibility for testing the correct ser/de behavior of that field type.
    //
    // Ser/de for each of these primitive types already has extensive test coverage in their own
    // unit tests, so repeating that coverage here is unnecessary.
    impl_message!(
        TestMessage1,
        a u8, b u16, c U24, d u32, e U256,
        f f32,
        g STR0_32, h STR0_255,
        i B0_31, j B0_32, k B0_255, l B0_64K, m B0_16M
    );

    impl TestMessage1 {
        fn new(
            a: u8,
            b: u16,
            c: U24,
            d: u32,
            e: U256,
            f: f32,
            g: STR0_32,
            h: STR0_255,
            i: B0_31,
            j: B0_32,
            k: B0_255,
            l: B0_64K,
            m: B0_16M,
        ) -> Result<TestMessage1> {
            Ok(TestMessage1 {
                a,
                b,
                c,
                d,
                e,
                f,
                g,
                h,
                i,
                j,
                k,
                l,
                m,
            })
        }
    }

    fn make_deserialized_test_message() -> TestMessage1 {
        TestMessage1::new(
            1u8,
            2u16,
            U24::new(3u32).unwrap(),
            4u32,
            U256([5u8; 32]),
            6.0f32,
            STR0_32::new("seven").unwrap(),
            STR0_255::new("eight").unwrap(),
            B0_31::new([9u8; 4]).unwrap(),
            B0_32::new([10u8; 4]).unwrap(),
            B0_255::new([11u8; 4]).unwrap(),
            B0_64K::new([12u8; 4]).unwrap(),
            B0_16M::new([13u8; 4]).unwrap(),
        )
        .unwrap()
    }

    fn make_serialized_test_message() -> Vec<u8> {
        return vec![
            0x01, // a
            0x02, 0x00, // b
            0x03, 0x00, 0x00, // c
            0x04, 0x00, 0x00, 0x00, // d
            0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
            0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
            0x05, 0x05, 0x05, 0x05, // e
            0x00, 0x00, 0xc0, 0x40, // f
            0x05, 0x73, 0x65, 0x76, 0x65, 0x6e, // g
            0x05, 0x65, 0x69, 0x67, 0x68, 0x74, // h
            0x04, 0x09, 0x09, 0x09, 0x09, // i
            0x04, 0x0a, 0x0a, 0x0a, 0x0a, // j
            0x04, 0x0b, 0x0b, 0x0b, 0x0b, // k
            0x04, 0x00, 0x0c, 0x0c, 0x0c, 0x0c, // l
            0x04, 0x00, 0x00, 0x0d, 0x0d, 0x0d, 0x0d, // m
        ];
    }

    impl_message_tests!(
        TestMessage1,
        make_serialized_test_message,
        make_deserialized_test_message
    );
}
