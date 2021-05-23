pub mod macro_prelude {
    pub use crate::error::{Error, Result};
    pub use crate::impl_message;
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::{MessageType, STR0_255};
    pub use std::convert::TryInto;
    pub use std::io;
}

/// Implemention of the requirements for a SetupConnection message for each
/// sub protocol.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_setup_connection {
    ($flags_type:ident) => {
        use crate::macro_message::setup_connection::macro_prelude::*;

        impl_message!(
            /// It's strongly recommended to use the [SetupConnection Enum](../common/setup_connection/enum.SetupConnection.html)
            /// and NOT this struct to initialize, serialize, deserialize and frame each SetupConnection
            /// message. Use this struct to extract the inner values of the message.
            ///
            /// SetupConnection is the first message sent by a client on a new connection.
            ///
            /// The SetupConnection struct contains all the common fields for the
            /// SetupConnection message required for each Stratum V2 subprotocol.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use stratumv2_lib::mining;
            /// use stratumv2_lib::job_negotiation;
            /// use stratumv2_lib::common::SetupConnection;
            ///
            /// let mining_conn = SetupConnection::new_mining(
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
            /// assert!(mining_conn.is_ok());
            ///
            /// let job_negotiation_conn = SetupConnection::new_job_negotation(
            ///    2,
            ///    2,
            ///    job_negotiation::SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING,
            ///    "0.0.0.0",
            ///    8545,
            ///    "Bitmain",
            ///    "S9i 13.5",
            ///    "braiins-os-2018-09-22-1-hash",
            ///    "some-device-uuid",
            /// );
            /// assert!(job_negotiation_conn.is_ok());
            ///
            /// if let SetupConnection::Mining(v) = mining_conn.unwrap() {
            ///     assert_eq!(v.min_version, 2);
            /// }
            ///
            /// if let SetupConnection::JobNegotiation(v) = job_negotiation_conn.unwrap() {
            ///     assert_eq!(v.min_version, 2);
            /// }
            ///
            /// ```
            SetupConnection,

            /// The minimum protocol version the client supports. (current default: 2)
            min_version u16,

            /// The maxmimum protocol version the client supports. (current default: 2)
            max_version u16,

            /// Flags indicating the optional protocol features the client supports.
            flags $flags_type,

            /// Used to indicate the hostname or IP address of the endpoint.
            endpoint_host STR0_255,

            /// Used to indicate the connecting port value of the endpoint.
            endpoint_port u16,

            /// The following fields relay the new_mining device information.
            ///
            /// Used to indicate the vendor/manufacturer of the device.
            vendor STR0_255,

            /// Used to indicate the hardware version of the device.
            hardware_version STR0_255,

            /// Used to indicate the firmware on the device.
            firmware STR0_255,

            /// Used to indicate the unique identifier of the device defined by the
            /// vendor.
            device_id STR0_255
        );

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
    };
}

#[cfg(test)]
pub mod test_macro_prelude {
    pub use crate::impl_message_tests;
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_setup_connection_tests {
    ($flags_type:ident) => {
        use crate::macro_message::setup_connection::test_macro_prelude::*;

        fn make_deserialized_setup_connection() -> SetupConnection {
            SetupConnection::new(
                2,
                2,
                $flags_type::all(),
                "0.0.0.0",
                8545,
                "Bitmain",
                "S9i 13.5",
                "braiins-os-2018-09-22-1-hash",
                "some-device-uuid",
            )
            .unwrap()
        }

        fn make_serialized_setup_connection() -> Vec<u8> {
            let mut serialized = vec![
                0x02, 0x00, // min_version
                0x02, 0x00, // max_version
            ];
            serialized.extend($flags_type::all().bits().to_le_bytes().iter()); // flags
            serialized.extend(vec![
                // endpoint_address
                0x07, // length (7)
                0x30, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x30, // "0.0.0.0"
            ]);
            serialized.extend(vec![
                0x61, 0x21, // endpoint_port
            ]);
            serialized.extend(vec![
                // vendor
                0x07, // length (7)
                0x42, 0x69, 0x74, 0x6d, 0x61, 0x69, 0x6e, // "Bitmain"
            ]);
            serialized.extend(vec![
                // hardware_version
                0x08, // length (8)
                0x53, 0x39, 0x69, 0x20, 0x31, 0x33, 0x2e, 0x35, // "S9i 13.5"
            ]);
            serialized.extend(vec![
                // firmware
                0x1c, // length (28)
                0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x2d, // "braiins-"
                0x6f, 0x73, 0x2d, 0x32, 0x30, 0x31, 0x38, 0x2d, // "os-2018-"
                0x30, 0x39, 0x2d, 0x32, 0x32, 0x2d, 0x31, 0x2d, // "09-22-1-"
                0x68, 0x61, 0x73, 0x68, // "hash"
            ]);
            serialized.extend(vec![
                // device_id
                0x10, // length (16)
                0x73, 0x6f, 0x6d, 0x65, 0x2d, 0x64, 0x65, 0x76, // "some-dev"
                0x69, 0x63, 0x65, 0x2d, 0x75, 0x75, 0x69, 0x64, // "ice-uuid"
            ]);

            serialized
        }

        impl_message_tests!(
            SetupConnection,
            make_serialized_setup_connection,
            make_deserialized_setup_connection
        );

        #[test]
        fn empty_vendor() {
            assert!(matches!(
                SetupConnection::new(
                    2,
                    2,
                    $flags_type::all(),
                    "0.0.0.0",
                    8545,
                    "",
                    "S9i 13.5",
                    "braiins-os-2018-09-22-1-hash",
                    "some-device-uuid",
                ),
                Err(Error::RequirementError { .. })
            ));
        }

        #[test]
        fn empty_firmware() {
            assert!(matches!(
                SetupConnection::new(
                    2,
                    2,
                    $flags_type::all(),
                    "0.0.0.0",
                    8545,
                    "Bitmain",
                    "S9i 13.5",
                    "",
                    "some-device-uuid",
                ),
                Err(Error::RequirementError { .. })
            ));
        }

        #[test]
        fn bad_min_version() {
            assert!(matches!(
                SetupConnection::new(
                    1,
                    2,
                    $flags_type::all(),
                    "0.0.0.0",
                    8545,
                    "Bitmain",
                    "S9i 13.5",
                    "braiins-os-2018-09-22-1-hash",
                    "some-device-uuid",
                ),
                Err(Error::VersionError { .. })
            ));
        }

        #[test]
        fn bad_max_version() {
            assert!(matches!(
                SetupConnection::new(
                    2,
                    1,
                    $flags_type::all(),
                    "0.0.0.0",
                    8545,
                    "Bitmain",
                    "S9i 13.5",
                    "braiins-os-2018-09-22-1-hash",
                    "some-device-uuid",
                ),
                Err(Error::VersionError { .. })
            ));
        }
    };
}
