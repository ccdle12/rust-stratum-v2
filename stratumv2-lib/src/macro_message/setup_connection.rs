pub mod macro_prelude {
    pub use crate::error::{Error, Result};
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::{MessageType, STR0_255};
    pub use std::convert::TryInto;
    pub use std::io;
}

/// Implemention of the requirements for a SetupConnection message for each
/// sub protocol.
#[macro_export]
macro_rules! impl_setup_connection {
    ($flags_type:ident) => {
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
        ///    2,
        ///    job_negotiation::SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING,
        ///    "0.0.0.0",
        ///    8545,
        ///    "Bitmain",
        ///    "S9i 13.5",
        ///    "braiins-os-2018-09-22-1-hash",
        ///    "some-device-uuid",
        /// );
        /// assert!(job_negotiation_connection.is_ok());
        /// ```
        #[derive(Debug, Clone)]
        pub struct SetupConnection {
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
    };
}
