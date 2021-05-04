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

#[macro_export]
macro_rules! impl_open_mining_channel_error {
    ($struct_name:ident, $msg_type:path) => {
        use crate::mining::open_mining_channel_error::macro_prelude::*;

        impl_message!(
            /// An implementation of the OpenMiningChannelError. This message applies to both
            /// Standard Mining Channels and Extended Mining Channels.
            $struct_name,
            $msg_type,
            /// A client specified request ID from the original OpenMiningChannel message.
            pub request_id u32,
            /// Pre-determined human readable error codes for the OpenMiningChannel message.
            pub error_code OpenMiningChannelErrorCode

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
