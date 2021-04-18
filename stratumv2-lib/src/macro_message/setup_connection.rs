pub mod macro_prelude {
    pub use crate::common::Protocol;
    pub use crate::error::{Error, Result};
    pub use crate::frame::Frameable;
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::{MessageType, STR0_255};
    pub use std::convert::TryInto;
    pub use std::io;
}

/// Implemention of the requirements for a SetupConnection message for each
/// sub protocol.
#[macro_export]
macro_rules! impl_setup_connection {
    ($protocol:path, $flags_type:ident) => {
        use crate::macro_message::setup_connection::macro_prelude::*;

        /// SetupConnection is the first message sent by a client on a new connection.
        ///
        /// The SetupConnection struct contains all the common fields for the
        /// SetupConnection message for each Stratum V2 subprotocol.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use stratumv2_lib::mining;
        /// use stratumv2_lib::job_negotiation;
        ///
        /// let mining_connection = mining::SetupConnection::new(
        ///    2,
        ///    2,
        ///    mining::SetupConnectionFlags::REQUIRES_STANDARD_JOBS | mining::SetupConnectionFlags::REQUIRES_VERSION_ROLLING,
        ///    "0.0.0.0",
        ///    8545,
        ///    "Bitmain",
        ///    "S9i 13.5",
        ///    "braiins-os-2018-09-22-1-hash",
        ///    "some-device-uuid",
        /// );
        /// assert!(mining_connection.is_ok());
        /// assert!(mining_connection.unwrap().flags.contains(
        ///     mining::SetupConnectionFlags::REQUIRES_STANDARD_JOBS)
        /// );
        ///
        /// let job_negotiation_connection = job_negotiation::SetupConnection::new(
        ///    2,
        ///    2, job_negotiation::SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING,
        ///    "0.0.0.0",
        ///    8545,
        ///    "Bitmain",
        ///    "S9i 13.5",
        ///    "braiins-os-2018-09-22-1-hash",
        ///    "some-device-uuid",
        /// );
        /// assert!(job_negotiation_connection.is_ok());
        /// ```
        #[derive(Debug, Clone, PartialEq)]
        pub struct SetupConnection {
            /// Used to indicate the protocol the client wants to use on the new connection.
            pub(crate) protocol: Protocol,

            /// The minimum protocol version the client supports. (current default: 2)
            pub min_version: u16,

            /// The maxmimum protocol version the client supports. (current default: 2)
            pub max_version: u16,

            /// Flags indicating the optional protocol features the client supports.
            pub flags: $flags_type,

            /// Used to indicate the hostname or IP address of the endpoint.
            pub endpoint_host: STR0_255,

            /// Used to indicate the connecting port value of the endpoint.
            pub endpoint_port: u16,

            /// The following fields relay the new_mining device information.
            ///
            /// Used to indicate the vendor/manufacturer of the device.
            pub vendor: STR0_255,

            /// Used to indicate the hardware version of the device.
            pub hardware_version: STR0_255,

            /// Used to indicate the firmware on the device.
            pub firmware: STR0_255,

            /// Used to indicate the unique identifier of the device defined by the
            /// vendor.
            pub device_id: STR0_255,
        }

        impl SetupConnection {
            pub fn new<T: Into<String>>(
                min_version: u16,
                max_version: u16,
                flags: $flags_type,
                endpoint_host: T,
                endpoint_port: u16,
                vendor: T,
                hardware_version: T,
                firmware: T,
                device_id: T,
            ) -> Result<SetupConnection> {
                let vendor = vendor.into();
                if *&vendor.is_empty() {
                    return Err(Error::RequirementError(
                        "vendor field in SetupConnection MUST NOT be empty".into(),
                    ));
                }

                let firmware = firmware.into();
                if *&firmware.is_empty() {
                    return Err(Error::RequirementError(
                        "firmware field in SetupConnection MUST NOT be empty".into(),
                    ));
                }

                if min_version < 2 {
                    return Err(Error::VersionError("min_version must be atleast 2".into()));
                }

                if max_version < 2 {
                    return Err(Error::VersionError("max_version must be atleast 2".into()));
                }

                Ok(SetupConnection {
                    protocol: $protocol,
                    min_version,
                    max_version,
                    flags,
                    endpoint_host: STR0_255::new(endpoint_host)?,
                    endpoint_port,
                    vendor: STR0_255::new(vendor)?,
                    hardware_version: STR0_255::new(hardware_version)?,
                    firmware: STR0_255::new(firmware)?,
                    device_id: STR0_255::new(device_id)?,
                })
            }
        }

        /// Implementation of the Serializable trait to serialize the contents
        /// of the SetupConnection message to the valid message format.
        impl Serializable for SetupConnection {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                Ok([
                    self.protocol.serialize(writer)?,
                    self.min_version.serialize(writer)?,
                    self.max_version.serialize(writer)?,
                    self.flags.serialize(writer)?,
                    self.endpoint_host.serialize(writer)?,
                    self.endpoint_port.serialize(writer)?,
                    self.vendor.serialize(writer)?,
                    self.hardware_version.serialize(writer)?,
                    self.firmware.serialize(writer)?,
                    self.device_id.serialize(writer)?,
                ]
                .iter()
                .sum())
            }
        }

        impl Deserializable for SetupConnection {
            fn deserialize(parser: &mut ByteParser) -> Result<SetupConnection> {
                let _protocol = Protocol::deserialize(parser)?;
                let min_version = u16::deserialize(parser)?;
                let max_version = u16::deserialize(parser)?;
                let flags = $flags_type::deserialize(parser)?;
                let endpoint_host = STR0_255::deserialize(parser)?;
                let endpoint_port = u16::deserialize(parser)?;
                let vendor = STR0_255::deserialize(parser)?;
                let hardware_version = STR0_255::deserialize(parser)?;
                let firmware = STR0_255::deserialize(parser)?;
                let device_id = STR0_255::deserialize(parser)?;

                SetupConnection::new(
                    min_version,
                    max_version,
                    flags,
                    endpoint_host,
                    endpoint_port,
                    vendor,
                    hardware_version,
                    firmware,
                    device_id,
                )
            }
        }

        impl Frameable for SetupConnection {
            fn message_type() -> MessageType {
                MessageType::SetupConnection
            }
        }
    };
}

#[cfg(test)]
macro_rules! impl_setup_connection_tests {
    ($type:ident, $empty_flag:expr, $flags:expr) => {
        use crate::common::Protocol;
        use crate::error::Result;
        use crate::frame::{frame, unframe, Message};
        use crate::parse::{deserialize, serialize};
        use crate::types::U24;
        use std::collections::HashMap;
        use std::convert::TryFrom;

        // Helper test function to generate a default SetupConnection message
        // with optional args passed through a HashpMap.
        fn default_setup_connection(args: HashMap<String, String>) -> Result<$type> {
            let mut min_version = 2;
            let mut max_version = 2;
            let mut vendor = "Bitmain";
            let mut firmware = "braiins-os-2018-09-22-1-hash";

            if args.contains_key("min_version") {
                min_version = args.get("min_version").unwrap().parse::<u16>().unwrap();
            }

            if args.contains_key("max_version") {
                max_version = args.get("max_version").unwrap().parse::<u16>().unwrap();
            }

            if args.contains_key("vendor") {
                vendor = args.get("vendor").unwrap();
            }

            if args.contains_key("firmware") {
                firmware = args.get("firmware").unwrap();
            }

            let message = $type::new(
                min_version,
                max_version,
                $flags,
                "0.0.0.0",
                8545,
                vendor,
                "S9i 13.5",
                firmware,
                "some-uuid",
            );

            message
        }

        #[test]
        fn setup_connection_invalid_min_version() {
            let mut args = HashMap::new();
            args.insert("min_version".into(), 1.to_string());

            let message = default_setup_connection(args);
            assert!(message.is_err());
        }

        #[test]
        fn setup_connection_invalid_max_version() {
            let mut args = HashMap::new();
            args.insert("max_version".into(), 0.to_string());

            let message = default_setup_connection(args);
            assert!(message.is_err());
        }

        #[test]
        fn setup_connection_empty_vendor() {
            let mut args = HashMap::new();
            args.insert("vendor".into(), "".to_string());

            let message = default_setup_connection(args);

            assert!(message.is_err())
        }

        #[test]
        fn setup_connection_empty_firmware() {
            let mut args = HashMap::new();
            args.insert("firmware".into(), "".to_string());

            let message = default_setup_connection(args);
            assert!(message.is_err())
        }

        #[test]
        fn serialize_setup_connection() {
            let message = default_setup_connection(HashMap::new()).unwrap();

            let buffer = serialize(&message).unwrap();
            assert_eq!(buffer.len(), 75);

            // Check the protocol enum was serialized correctly.
            assert_eq!(Protocol::try_from(buffer[0]).unwrap(), message.protocol);

            // Check the flags were serialized correctly.
            assert_eq!(buffer[5..9], serialize(&message.flags).unwrap());

            // Sanity check - deserializing back to the struct does not cause
            // errors.
            assert!(deserialize::<$type>(&buffer).is_ok());
        }

        #[test]
        fn serialize_empty_flags() {
            let message = $type::new(
                2,
                2,
                $empty_flag,
                "0.0.0.0",
                8545,
                "Bitmain",
                "S9i 13.5",
                "braiins-os-2018-09-22-1-hash",
                "some-uuid",
            )
            .unwrap();

            let buffer = serialize(&message).unwrap();
            assert_eq!(buffer.len(), 75);

            // Check that optional flags were serialized correctly when empty.
            assert_eq!(buffer[5..9], [0u8; 4]);
        }

        #[test]
        fn frame_setup_connection() {
            let message = default_setup_connection(HashMap::new()).unwrap();

            let network_message = frame(&message).unwrap();
            let buffer = serialize(&network_message).unwrap();
            assert_eq!(buffer.len(), 81);

            // Check the extension type is empty.
            assert_eq!(buffer[0..2], [0u8; 2]);

            // Check that the correct byte for the message type was used.
            assert_eq!(buffer[2], network_message.message_type.msg_type());

            // Check that the correct message length was used.
            assert_eq!(
                deserialize::<U24>(&buffer[3..6]).unwrap(),
                network_message.payload.len() as u32
            );

            // Check that the network message bytes can be deserialized and
            // unframed back into the SetupConnection message.
            let der_network_message = deserialize::<Message>(&buffer).unwrap();
            let der_message = unframe::<$type>(&network_message).unwrap();
            assert_eq!(der_message, message);
        }
    };
}

#[cfg(test)]
mod mining_connection_tests {
    use crate::mining::SetupConnection;
    use crate::mining::SetupConnectionFlags;

    impl_setup_connection_tests!(
        SetupConnection,
        SetupConnectionFlags::empty(),
        SetupConnectionFlags::REQUIRES_STANDARD_JOBS
            | SetupConnectionFlags::REQUIRES_WORK_SELECTION
    );
}

#[cfg(test)]
mod job_negotiation_tests {
    use crate::job_negotiation::SetupConnection;
    use crate::job_negotiation::SetupConnectionFlags;

    impl_setup_connection_tests!(
        SetupConnection,
        SetupConnectionFlags::empty(),
        SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING
    );
}
