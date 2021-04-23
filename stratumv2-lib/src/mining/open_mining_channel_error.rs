use crate::impl_error_code_enum;

/// Contains the error codes for the [OpenStandardMiningChannelError](struct.OpenStandardMiningChannelError.html)
/// and [OpenExtendedMiningChannelError](struct.OpenExtendedMiningChannelError.html)
/// message. Each error code is serialized according to constraints of a
/// [STR0_32](../types/struct.STR0_32.html).
#[derive(Debug, PartialEq)]
pub enum OpenMiningChannelErrorCode {
    UnknownUser,
    MaxTargetOutOfRange,
}

impl_error_code_enum!(
    OpenMiningChannelErrorCode,
    OpenMiningChannelErrorCode::UnknownUser => "unknown-user",
    OpenMiningChannelErrorCode::MaxTargetOutOfRange => "max-target-out-of-range"

);

pub mod macro_prelude {
    pub use super::OpenMiningChannelErrorCode;
    pub use crate::error::Result;
    pub use crate::frame::Frameable;
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::{MessageType, B0_32, U256};
    pub use std::io;
}

/// Implementation of the OpenMiningChannelError. This message applies to both
/// Standard Mining Channels and Extended Mining Channels.
#[macro_export]
macro_rules! impl_open_mining_channel_error {
    ($name:ident, $msg_type:path) => {
        use crate::mining::open_mining_channel_error::macro_prelude::*;

        pub struct $name {
            request_id: u32,
            error_code: OpenMiningChannelErrorCode,
        }

        impl $name {
            pub fn new(request_id: u32, error_code: OpenMiningChannelErrorCode) -> $name {
                $name {
                    request_id,
                    error_code,
                }
            }
        }

        impl Serializable for $name {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                Ok([
                    self.request_id.serialize(writer)?,
                    self.error_code.serialize(writer)?,
                ]
                .iter()
                .sum())
            }
        }

        impl Deserializable for $name {
            fn deserialize(parser: &mut ByteParser) -> Result<$name> {
                Ok($name::new(
                    u32::deserialize(parser)?,
                    OpenMiningChannelErrorCode::deserialize(parser)?,
                ))
            }
        }

        impl Frameable for $name {
            fn message_type() -> MessageType {
                $msg_type
            }
        }
    };
}
