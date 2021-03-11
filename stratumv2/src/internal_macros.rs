/// A convenience macro for serializing a variable length of byte slices to a
/// Vector.
macro_rules! serialize_slices {
    ($($x:expr),*) => {{
        let mut buffer: Vec<u8> = Vec::new();
        $( buffer.extend_from_slice($x);)*
        buffer
    }};
}

/// An internal macro for implementing the From trait for existing Error types
/// into the projects Error type variants.
macro_rules! impl_error_conversions {
    ($($error_type:path => $error_variant:path),*) => {
        $(impl From<$error_type> for Error {
            fn from(err: $error_type) -> Error {
                $error_variant(err)
            }
        })*
    };
}

/// An internal helper macro for getting the unix time now as a u32.
macro_rules! unix_u32_now {
    () => {
        system_unix_time_to_u32(&SystemTime::now())
    };
}

/// An internal macro to implement the Frameable trait for messages. Some mesages
/// require the extenstion type to have a channel_msg bit set since the message
/// is intended for a specific channel_id. The channel_id will always be found
/// in the deserialized object as a field.
macro_rules! impl_frameable_trait {
    ($msg:ident, $msg_type:path, $has_channel_msg_bit:expr) => {
        impl Frameable for $msg {
            internal_frameable_trait!($msg_type, $has_channel_msg_bit);
        }
    };
}

macro_rules! impl_frameable_trait_with_liftime {
    ($msg:ident, $msg_type:path, $has_channel_msg_bit:expr, $lt:lifetime) => {
        impl<$lt> Frameable for $msg<$lt> {
            internal_frameable_trait!($msg_type, $has_channel_msg_bit);
        }
    };
}

// TODO: Implement channel_msg branch.
macro_rules! internal_frameable_trait {
    ($msg_type:path, $has_channel_msg_bit:expr) => {
        fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
            let mut payload = Vec::new();
            let size = *&self.serialize(&mut payload)?;

            // A size_u24 of the message payload.
            let payload_length = (size as u32).to_le_bytes()[0..=2].to_vec();

            let buffer = serialize_slices!(
                &[0x00, 0x00],       // empty extension type
                &[$msg_type.into()], // msg_type
                &payload_length,
                &payload
            );

            Ok(writer.write(&buffer)?)
        }
    };
}

/// An internal macro that implements a STR0 type that is restricted according
/// to a MAX_SIZE.
macro_rules! impl_sized_STR0 {
    ($type:ident, $max_size:expr) => {
        #[derive(Debug, Clone)]
        pub struct $type(pub(crate) String);

        impl $type {
            const MAX_SIZE: usize = $max_size;

            /// The constructor enforces the String input size as the MAX_SIZE.
            /// A RequirementError will be returned if the input byte size is
            /// greater than the MAX_SIZE.
            pub fn new<T: Into<String>>(value: T) -> Result<$type> {
                let value = value.into();
                if value.len() > Self::MAX_SIZE {
                    return Err(Error::RequirementError(
                        "string size cannot be greater than MAX_SIZE".into(),
                    ));
                }

                Ok($type(value))
            }

            /// Returns the byte representation. Specifically it returns the
            /// byte representation for serializing according to the protocol
            /// specification which is <1 byte length prefix + variable length STR0_255>.
            pub fn as_bytes(&self) -> Vec<u8> {
                serialize_slices!(&[self.0.len() as u8], self.0.as_bytes())
            }
        }

        /// PartialEq implementation allowing direct comparison between the STR0 type
        /// and String.
        impl PartialEq<String> for $type {
            fn eq(&self, other: &String) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$type> for String {
            fn eq(&self, other: &$type) -> bool {
                *self == other.0
            }
        }

        /// PartialEq implementation allowing direct comparison between STR0 types.
        impl PartialEq<$type> for $type {
            fn eq(&self, other: &$type) -> bool {
                *self.0 == other.0
            }
        }

        /// From trait implementation that allows a STR0 to be converted into a
        /// String.
        impl From<$type> for String {
            fn from(s: $type) -> Self {
                s.0
            }
        }
    };
}
