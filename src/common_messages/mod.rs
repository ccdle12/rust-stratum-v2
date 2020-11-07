use crate::error::{Error, Result};

/// SetupConnection is the first message sent by a client on a new connection.
pub struct SetupConnection {
    /// Used to indicate the protocol the client wants to use on the new connection.
    /// Available protocols:
    ///   0 = Mining Protocol
    ///   1 = Job Negotiation Protocol
    ///   2 = Template Distribution Protocol
    ///   3 = Job Distribution Protocol
    protocol: u8,

    /// The minimum protocol version the client supports. (current default: 2)
    min_version: u16,

    /// The maxmimum protocol version the client supports. (current default: 2)
    max_version: u16,

    /// Flags indicating the optional protocol features the client supports.
    flags: u32,

    /// Used to indicate the hostname or IP address of the endpoint.
    endpoint_host: String,

    /// Used to indicate the connecting port value of the endpoint.
    endpoint_port: u16,

    /// The following fields relay the mining device information.
    ///
    /// Used to indicate the vendor/manufacturer of the device.
    vendor: String,

    /// Used to indicate the hardware version of the device.
    hardware_version: String,

    /// Used to indicate the firmware on the device.
    firmware: String,

    /// Used to indicate the unique identifier of the device defined by the
    /// vendor.
    device_id: String,
}

impl SetupConnection {
    /// Constructor for the SetupConnection message.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use stratumv2::SetupConnection;
    ///
    /// let connection_msg = SetupConnection::new(
    ///     0,
    ///     2,
    ///     2,
    ///     123,
    ///     "0.0.0.0",
    ///     8545,
    ///     "Bitmain",
    ///     "S9i 13.5",
    ///     "braiins-os-2018-09-22-1-hash",
    ///     "some-uuid",
    /// );
    ///
    /// assert_eq!(connection_msg.is_err(), false);
    /// ```
    pub fn new<T: Into<String>>(
        protocol: u8,
        min_version: u16,
        max_version: u16,
        flags: u32,
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<Self> {
        if protocol > 3 {
            return Err(Error::VersionError("invalid protocol_version".into()));
        }

        if min_version < 2 {
            return Err(Error::VersionError("min_version must be atleast 2".into()));
        }

        if max_version < 2 {
            return Err(Error::VersionError("max_version must be atleast 2".into()));
        }

        Ok(SetupConnection {
            protocol,
            min_version,
            max_version,
            flags,
            endpoint_host: endpoint_host.into(),
            endpoint_port,
            vendor: vendor.into(),
            hardware_version: hardware_version.into(),
            firmware: firmware.into(),
            device_id: device_id.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_connection_init() {
        let connection_msg = SetupConnection::new(
            0,
            2,
            2,
            123,
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert_eq!(connection_msg.is_err(), false);
    }

    #[test]
    fn setup_connection_invalid_min_value() {
        let connection_msg = SetupConnection::new(
            0,
            0,
            2,
            123,
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert_eq!(connection_msg.is_err(), true);
    }

    #[test]
    fn setup_connection_invalid_max_value() {
        let connection_msg = SetupConnection::new(
            0,
            2,
            0,
            123,
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert_eq!(connection_msg.is_err(), true);
    }

    #[test]
    fn setup_connection_invalid_protocol_version() {
        let connection_msg = SetupConnection::new(
            4,
            2,
            2,
            123,
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert_eq!(connection_msg.is_err(), true);
    }
}
