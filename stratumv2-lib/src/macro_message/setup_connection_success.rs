pub mod macro_prelude {
    pub use crate::error::{Error, Result};
    pub use crate::frame::Frameable;
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::MessageType;
    pub use std::io;
}

#[macro_export]
macro_rules! impl_setup_connection_success {
    ($flags_type:ident) => {
        use crate::macro_message::setup_connection_success::macro_prelude::*;

        /// SetupConnectionSuccess is one of the required responses from a
        /// Server to a Client when a connection is accepted.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use stratumv2_lib::mining;
        ///
        /// let conn_success = mining::SetupConnectionSuccess::new(
        ///    2,
        ///    mining::SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION,
        /// );
        /// assert_eq!(
        ///     conn_success.flags,
        ///     mining::SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION
        /// );
        /// ```
        pub struct SetupConnectionSuccess {
            /// Version proposed by the connecting node as one of the verions supported
            /// by the upstream node. The version will be used during the lifetime of
            /// the connection.
            pub used_version: u16,

            /// Indicates the optional features the server supports.
            pub flags: $flags_type,
        }

        impl SetupConnectionSuccess {
            /// Constructor for the SetupConnectionSuccess message.
            pub fn new(used_version: u16, flags: $flags_type) -> SetupConnectionSuccess {
                SetupConnectionSuccess {
                    used_version,
                    flags,
                }
            }
        }

        impl Serializable for SetupConnectionSuccess {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let length =
                    self.used_version.serialize(writer)? + self.flags.bits().serialize(writer)?;

                Ok(length)
            }
        }

        impl Deserializable for SetupConnectionSuccess {
            fn deserialize(parser: &mut ByteParser) -> Result<SetupConnectionSuccess> {
                let used_version = u16::deserialize(parser)?;
                let flags = $flags_type::from_bits(u32::deserialize(parser)?)
                    .ok_or(Error::UnknownFlags())?;

                Ok(SetupConnectionSuccess {
                    used_version: used_version,
                    flags: flags,
                })
            }
        }

        impl Frameable for SetupConnectionSuccess {
            fn message_type() -> MessageType {
                MessageType::SetupConnectionSuccess
            }
        }
    };
}
