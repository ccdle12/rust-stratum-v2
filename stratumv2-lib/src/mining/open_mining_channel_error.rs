use crate::impl_error_code_enum;

/// Contains the error codes for the [OpenStandardMiningChannelError](struct.OpenStandardMiningChannelError.html)
/// and [OpenExtendedMiningChannelError](struct.OpenExtendedMiningChannelError.html)
/// message. Each error code is serialized according to constraints of a
/// [STR0_32](../types/struct.STR0_32.html).
#[derive(Debug, Clone, PartialEq)]
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
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_open_mining_channel_error {
    ($struct_name:ident) => {
        use crate::mining::open_mining_channel_error::macro_prelude::*;

        impl_message!(
            /// An implementation of the OpenMiningChannelError. This message applies to both
            /// Standard Mining Channels and Extended Mining Channels.
            $struct_name,

            /// A client specified request ID from the original OpenMiningChannel message.
            request_id u32,

            /// Pre-determined human readable error codes for the OpenMiningChannel message.
            error_code OpenMiningChannelErrorCode

        );

        impl $struct_name {
            pub fn new(request_id: u32, error_code: OpenMiningChannelErrorCode) -> Result<$struct_name> {
                Ok($struct_name {
                    request_id,
                    error_code,
                })
            }
        }
    };
}

#[cfg(test)]
pub mod test_macro_prelude {
    pub use crate::impl_message_tests;
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_open_mining_channel_error_tests {
    ($struct_name:ident) => {
        use crate::mining::open_mining_channel_error::test_macro_prelude::*;

        fn make_deserialized_open_mining_channel_error() -> $struct_name {
            $struct_name::new(0x01, OpenMiningChannelErrorCode::UnknownUser).unwrap()
        }

        fn make_serialized_open_mining_channel_error() -> Vec<u8> {
            let mut serialized = vec![0x01, 0x00, 0x00, 0x00]; // request_id
            serialized.extend(vec![
                // error_code
                0x0c, 0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x2d, 0x75, 0x73, 0x65, 0x72,
            ]);

            serialized
        }

        impl_message_tests!(
            $struct_name,
            make_serialized_open_mining_channel_error,
            make_deserialized_open_mining_channel_error
        );
    };
}
