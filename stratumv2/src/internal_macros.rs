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
    ($msg:ident, $type:path, $has_channel_msg_bit:expr) => {
        impl Frameable for $msg {
            internal_frameable_trait!($type, $has_channel_msg_bit);
        }
    };
}

macro_rules! impl_frameable_trait_with_liftime {
    ($msg:ident, $type:path, $has_channel_msg_bit:expr) => {
        impl<'a> Frameable for $msg<'a> {
            internal_frameable_trait!($type, $has_channel_msg_bit);
        }
    };
}

// TODO: Implement channel_msg branch.
macro_rules! internal_frameable_trait {
    ($type:path, $has_channel_msg_bit:expr) => {
        fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
            let mut payload = Vec::new();
            let size = *&self.serialize(&mut payload)?;

            // A size_u24 of the message payload.
            let payload_length = (size as u32).to_le_bytes()[0..=2].to_vec();

            let buffer = serialize_slices!(
                &[0x00, 0x00],   // empty extension type
                &[$type.into()], // msg_type
                &payload_length,
                &payload
            );

            Ok(writer.write(&buffer)?)
        }
    };
}
