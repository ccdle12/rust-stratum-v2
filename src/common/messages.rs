use crate::common::Serializable;
use crate::common::{BitFlag, ToProtocol};
use crate::error::{Error, Result};
use crate::mining;
use crate::util::types::{string_to_str0_255, string_to_str0_255_bytes};
use std::fmt;
use std::io;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
/// Protocol is an enum representing each sub protocol of Stratum V2.
pub enum Protocol {
    /// Mining is the main and only required sub protocol in Stratum V2.
    Mining = 0,

    /// JobNegotiation is a protocol for intermediate nodes to intermediate
    /// the terms of a connection between downstream nodes and upstream nodes.
    JobNegotiation = 1,
    TemplateDistribution = 2,
    JobDistribution = 3,
}

/// SetupConnection is the first message sent by a client on a new connection.
/// This implementation is a base struct that contains all the common fields
/// for the SetupConnection for each Stratum V2 subprotocol.
pub struct SetupConnection<'a, B> {
    /// Used to indicate the protocol the client wants to use on the new connection.
    pub protocol: Protocol,

    /// The minimum protocol version the client supports. (current default: 2)
    pub min_version: u16,

    /// The maxmimum protocol version the client supports. (current default: 2)
    pub max_version: u16,

    /// Flags indicating the optional protocol features the client supports.
    pub flags: &'a [B],

    /// Used to indicate the hostname or IP address of the endpoint.
    pub endpoint_host: String,

    /// Used to indicate the connecting port value of the endpoint.
    pub endpoint_port: u16,

    /// The following fields relay the new_mining device information.
    ///
    /// Used to indicate the vendor/manufacturer of the device.
    pub vendor: String,

    /// Used to indicate the hardware version of the device.
    pub hardware_version: String,

    /// Used to indicate the firmware on the device.
    pub firmware: String,

    /// Used to indicate the unique identifier of the device defined by the
    /// vendor.
    pub device_id: String,
}

impl<'a, B> SetupConnection<'a, B>
where
    B: BitFlag + ToProtocol,
{
    /// Internal constructor for the SetupConnection message. Each subprotcol
    /// has its own public setup connection method that should be called.
    pub fn new<T: Into<String>>(
        protocol: Protocol,
        min_version: u16,
        max_version: u16,
        flags: &'a [B],
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection<B>> {
        for flag in flags {
            if flag.as_protocol() != protocol {
                return Err(Error::VersionError("flags do not match protocol".into()));
            }
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

impl<B> Serializable for SetupConnection<'_, B>
where
    B: BitFlag,
{
    /// Implementation of the Serializable trait to serialize the contents
    /// of the SetupConnection message to the valid message format.
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = vec![self.protocol as u8];

        buffer.extend_from_slice(&self.min_version.to_le_bytes());
        buffer.extend_from_slice(&self.max_version.to_le_bytes());

        let byte_flags = (self
            .flags
            .iter()
            .map(|x| x.as_byte())
            .fold(0, |accumulator, byte| (accumulator | byte)) as u32)
            .to_le_bytes();

        buffer.extend_from_slice(&byte_flags);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.endpoint_host)?);
        buffer.extend_from_slice(&self.endpoint_port.to_le_bytes());
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.vendor)?);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.hardware_version)?);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.firmware)?);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.device_id)?);

        Ok(writer.write(&buffer)?)
    }
}

/// SetupConnectionSuccess is one of the required responses from a
/// Server to a Client when a connection is accepted.
pub struct SetupConnectionSuccess {
    /// Version proposed by the connecting node that the upstream node (Server?)
    /// supports. The version will be used during the lifetime of the connection.
    used_version: u16,

    /// Used to indicate the optional features the server supports.
    flags: Vec<u8>,
}

impl SetupConnectionSuccess {
    /// Constructor for the SetupConnectionSuccess message for the mining protocol.
    pub fn new_mining_success(
        used_version: u16,
        flags: &[mining::SetupConnectionSuccessFlags],
    ) -> SetupConnectionSuccess {
        let flags: Vec<u8> = flags.iter().map(|x| x.as_byte()).collect();
        SetupConnectionSuccess {
            used_version,
            flags,
        }
    }
}

impl Serializable for SetupConnectionSuccess {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.extend_from_slice(&self.used_version.to_le_bytes());

        let byte_flags = (self
            .flags
            .iter()
            .fold(0, |accumulator, byte| (accumulator | byte)) as u32)
            .to_le_bytes();

        buffer.extend_from_slice(&byte_flags);

        Ok(writer.write(&buffer)?)
    }
}

/// Error codes for the `SetupConnectionError` message. Each error code has
/// a default STR0_255 message.
pub enum SetupConnectionErrorCodes {
    /// Indicates the server has received a feature flag from a client that
    /// the server does not support.
    UnsupportedFeatureFlags,

    /// Indicates the server has received a connection request using a protcol
    /// the server does not support.
    UnsupportedProtocol,

    // TODO: What is the difference between protocol version mismatch
    // and unsupported protocol?
    ProtocolVersionMismatch,
}

impl fmt::Display for SetupConnectionErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetupConnectionErrorCodes::UnsupportedFeatureFlags => {
                write!(f, "unsupported-feature-flags")
            }
            SetupConnectionErrorCodes::UnsupportedProtocol => write!(f, "unsupported-protocol"),
            SetupConnectionErrorCodes::ProtocolVersionMismatch => {
                write!(f, "protocol-version-mismatch")
            }
        }
    }
}

/// SetupConnectionError is one of the required respones from a server to client
/// when a new connection has failed. The server is required to send this message
/// with an error code before closing the connection.
///
/// If the error is a `FeatureFlag` error, the server MUST respond with a all
/// the feature flags that it does not support.
pub struct SetupConnectionError<'a, B> {
    flags: &'a [B],
    error_code: SetupConnectionErrorCodes,
}

impl<B> SetupConnectionError<'_, B>
where
    B: BitFlag,
{
    /// Constructor for the SetupConnectionError message.
    pub fn new(flags: &[B], error_code: SetupConnectionErrorCodes) -> SetupConnectionError<B> {
        SetupConnectionError {
            flags: &flags,
            error_code,
        }
    }
}

impl<B> Serializable for SetupConnectionError<'_, B>
where
    B: BitFlag,
{
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = Vec::new();

        let byte_flags = (self
            .flags
            .iter()
            .map(|x| x.as_byte())
            .fold(0, |accumulator, byte| (accumulator | byte)) as u32)
            .to_le_bytes();

        buffer.extend_from_slice(&byte_flags);
        buffer.extend_from_slice(&string_to_str0_255_bytes(&self.error_code.to_string())?);

        Ok(writer.write(&buffer)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Serializable;
    use crate::job_negotiation;

    #[test]
    fn setup_connection_init() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_ok());
    }

    #[test]
    fn setup_connection_invalid_flag_and_protcol() {
        let message = SetupConnection::new(
            Protocol::JobNegotiation,
            2,
            2,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_err());
    }

    #[test]
    fn setup_connection_invalid_min_value() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            1,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_err());
    }

    #[test]
    fn setup_connection_invalid_max_value() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            0,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_err());
    }

    #[test]
    fn setup_connection_success() {
        let message = SetupConnectionSuccess::new_mining_success(2, &[]);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_serialize_0() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();

        let size = message.serialize(&mut buffer).unwrap();
        assert_eq!(size, 75);

        let expected = [
            0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x07, 0x30, 0x2e, 0x30, 0x2e,
            0x30, 0x2e, 0x30, 0x61, 0x21, 0x07, 0x42, 0x69, 0x74, 0x6d, 0x61, 0x69, 0x6e, 0x08,
            0x53, 0x39, 0x69, 0x20, 0x31, 0x33, 0x2e, 0x35, 0x1c, 0x62, 0x72, 0x61, 0x69, 0x69,
            0x6e, 0x73, 0x2d, 0x6f, 0x73, 0x2d, 0x32, 0x30, 0x31, 0x38, 0x2d, 0x30, 0x39, 0x2d,
            0x32, 0x32, 0x2d, 0x31, 0x2d, 0x68, 0x61, 0x73, 0x68, 0x09, 0x73, 0x6f, 0x6d, 0x65,
            0x2d, 0x75, 0x75, 0x69, 0x64,
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_serialize_1() {
        let message: SetupConnection<'_, mining::SetupConnectionFlags> = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);

        // Expect the feature flag to have no set flags (0x00).
        assert_eq!(buffer[5], 0x00);
    }

    #[test]
    fn mining_setup_connection_serialize_2() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[
                mining::SetupConnectionFlags::RequiresStandardJobs,
                mining::SetupConnectionFlags::RequiresVersionRolling,
            ],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);
        assert_eq!(buffer[5], 0x05);
    }

    #[test]
    fn mining_setup_connection_serialize_3() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[
                mining::SetupConnectionFlags::RequiresStandardJobs,
                mining::SetupConnectionFlags::RequiresWorkSelection,
                mining::SetupConnectionFlags::RequiresVersionRolling,
            ],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);
        assert_eq!(buffer[5], 0x07);
    }

    #[test]
    fn new_job_negotiation_setup_connection_init() {
        let message = SetupConnection::new(
            Protocol::JobNegotiation,
            2,
            2,
            &[job_negotiation::SetupConnectionFlags::RequiresAsyncJobMining],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_ok());
    }

    #[test]
    fn new_job_negotiation_serialize_0() {
        let message = SetupConnection::new(
            Protocol::JobNegotiation,
            2,
            2,
            &[job_negotiation::SetupConnectionFlags::RequiresAsyncJobMining],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);
        assert_eq!(buffer[0], 0x01);
        assert_eq!(buffer[5], 0x01);
    }

    #[test]
    fn new_job_negotiation_serialize_1() {
        let message: SetupConnection<'_, job_negotiation::SetupConnectionFlags> =
            SetupConnection::new(
                Protocol::JobNegotiation,
                2,
                2,
                &[],
                "0.0.0.0",
                8545,
                "Bitmain",
                "S9i 13.5",
                "braiins-os-2018-09-22-1-hash",
                "some-uuid",
            )
            .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);
        assert_eq!(buffer[0], 0x01);
        assert_eq!(buffer[5], 0x00);
    }

    #[test]
    fn mining_setup_connection_success_0() {
        let message = SetupConnectionSuccess::new_mining_success(
            2,
            &[mining::SetupConnectionSuccessFlags::RequiresFixedVersion],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_success_1() {
        let message = SetupConnectionSuccess::new_mining_success(
            2,
            &[
                mining::SetupConnectionSuccessFlags::RequiresFixedVersion,
                mining::SetupConnectionSuccessFlags::RequiresExtendedChannels,
            ],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x03, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_success_2() {
        let message = SetupConnectionSuccess::new_mining_success(2, &[]);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn setup_connection_error_serialize_0() {
        let flags = &[mining::SetupConnectionFlags::RequiresStandardJobs];
        let message =
            SetupConnectionError::new(flags, SetupConnectionErrorCodes::UnsupportedFeatureFlags);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        // Feature flag.
        assert_eq!(buffer[0], 0x01);

        // Length of error code string.
        assert_eq!(buffer[4], 0x19);
    }
}
