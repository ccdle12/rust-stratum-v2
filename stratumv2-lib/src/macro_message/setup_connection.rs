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
            MessageType::SetupConnection,
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
