use crate::error::{Error, Result};
use crate::messages::Serializable;
use crate::util::types::{string_to_str0_255, string_to_str0_255_bytes};
use std::io;

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
    /// # use stratumv2::messages::common::SetupConnection;
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
            endpoint_host: string_to_str0_255(endpoint_host)?,
            endpoint_port,
            vendor: string_to_str0_255(vendor)?,
            hardware_version: string_to_str0_255(hardware_version)?,
            firmware: string_to_str0_255(firmware)?,
            device_id: string_to_str0_255(device_id)?,
        })
    }
}

impl Serializable for SetupConnection {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = vec![self.protocol];

        buffer.extend_from_slice(&self.min_version.to_le_bytes());
        buffer.extend_from_slice(&self.max_version.to_le_bytes());
        buffer.extend_from_slice(&self.flags.to_le_bytes());

        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.endpoint_host)?);
        buffer.extend_from_slice(&self.endpoint_port.to_le_bytes());

        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.vendor)?);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.hardware_version)?);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.firmware)?);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.device_id)?);

        Ok(writer.write(&buffer)?)
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

    #[test]
    fn setup_connection_serialize_0() {
        let connection_msg = SetupConnection::new(
            3,
            2,
            2,
            0,
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();

        let size = connection_msg.serialize(&mut buffer).unwrap();
        assert_eq!(size, 75);

        let expected = [
            0x03, 0x02, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x30, 0x2e, 0x30, 0x2e,
            0x30, 0x2e, 0x30, 0x61, 0x21, 0x07, 0x42, 0x69, 0x74, 0x6d, 0x61, 0x69, 0x6e, 0x08,
            0x53, 0x39, 0x69, 0x20, 0x31, 0x33, 0x2e, 0x35, 0x1c, 0x62, 0x72, 0x61, 0x69, 0x69,
            0x6e, 0x73, 0x2d, 0x6f, 0x73, 0x2d, 0x32, 0x30, 0x31, 0x38, 0x2d, 0x30, 0x39, 0x2d,
            0x32, 0x32, 0x2d, 0x31, 0x2d, 0x68, 0x61, 0x73, 0x68, 0x09, 0x73, 0x6f, 0x6d, 0x65,
            0x2d, 0x75, 0x75, 0x69, 0x64,
        ];
        assert_eq!(buffer, expected);
    }
}
