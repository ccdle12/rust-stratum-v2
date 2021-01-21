use crate::common::types::MessageTypes;
use crate::common::types::STR0_255;
use crate::common::{BitFlag, Framable, Protocol, Serializable};
use crate::error::{Error, Result};
use std::{fmt, io, str};

pub(crate) struct RawSetupConnection {
    /// Used to indicate the protocol the client wants to use on the new connection.
    pub(crate) protocol: Protocol,

    /// The minimum protocol version the client supports. (current default: 2)
    pub(crate) min_version: u16,

    /// The maxmimum protocol version the client supports. (current default: 2)
    pub(crate) max_version: u16,

    /// Flags indicating the optional protocol features the client supports.
    pub(crate) flags: u32,

    /// Used to indicate the hostname or IP address of the endpoint.
    pub(crate) endpoint_host: STR0_255,

    /// Used to indicate the connecting port value of the endpoint.
    pub(crate) endpoint_port: u16,

    /// The following fields relay the new_mining device information.
    ///
    /// Used to indicate the vendor/manufacturer of the device.
    pub(crate) vendor: STR0_255,

    /// Used to indicate the hardware version of the device.
    pub(crate) hardware_version: STR0_255,

    /// Used to indicate the firmware on the device.
    pub(crate) firmware: STR0_255,

    /// Used to indicate the unique identifier of the device defined by the
    /// vendor.
    pub(crate) device_id: STR0_255,
}

impl RawSetupConnection {
    pub(crate) fn new<T: Into<String>>(
        protocol: Protocol,
        min_version: u16,
        max_version: u16,
        flags: u32,
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<RawSetupConnection> {
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

        Ok(RawSetupConnection {
            protocol,
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

    /// Internal function to serialize the RawSetupConnection.
    pub(crate) fn serialize(&self) -> Vec<u8> {
        serialize!(
            &[self.protocol as u8],
            &self.min_version.to_le_bytes(),
            &self.max_version.to_le_bytes(),
            &self.flags.to_le_bytes(),
            &self.endpoint_host.as_bytes(),
            &self.endpoint_port.to_le_bytes(),
            &self.vendor.as_bytes(),
            &self.hardware_version.as_bytes(),
            &self.firmware.as_bytes(),
            &self.device_id.as_bytes()
        )
    }

    // TODO: Fuzz test this
    // Malformed bytes that don't contain the expected sized will raise
    // errors.
    /// Internal function to deserialize the RawSetupConnection.
    pub(crate) fn deserialize(bytes: &[u8]) -> Result<RawSetupConnection> {
        // Read the protocol bytes.
        let start = 0;
        let protocol_byte = &bytes[start];
        let protocol = Protocol::from(*protocol_byte);

        // Read the min_version.
        let start = 1;
        let offset = start + 2;
        let min_version_bytes = &bytes[start..offset];
        let min_version = (min_version_bytes[1] as u16) << 8 | min_version_bytes[0] as u16;

        // Read the max_version.
        let start = offset;
        let offset = start + 2;
        let max_version_bytes = &bytes[start..offset];
        let max_version = (max_version_bytes[1] as u16) << 8 | max_version_bytes[0] as u16;

        // Read the flags.
        let start = offset;
        let offset = start + 4;
        let flags_bytes = &bytes[start..offset];
        let flags = flags_bytes // This works because we just need set flags
            .iter()
            .map(|x| *x as u32)
            .fold(0, |accumulator, byte| (accumulator | byte));

        // Read the endpoint_host.
        let mut start = offset;
        let endpoint_host_length = *&bytes[start] as usize;
        start += 1;
        let offset = start + endpoint_host_length;
        let endpoint_host = &bytes[start..offset];

        // Read the endpoint_port.
        let start = offset;
        let offset = start + 2;
        let endpoint_port_bytes = &bytes[start..offset];

        // This works because we need accurate repr of numbers
        let endpoint_port = (endpoint_port_bytes[1] as u16) << 8 | endpoint_port_bytes[0] as u16;

        // Read the vendor.
        let mut start = offset;
        let vendor_length = *&bytes[start] as u8;
        start += 1;
        let offset = start as u8 + vendor_length;
        let vendor = &bytes[start..offset as usize];

        // Read the hardware version.
        let mut start = offset;
        let hardware_version_length = *&bytes[start as usize] as u8;
        start += 1;
        let offset = start + hardware_version_length;
        let hardware_version = &bytes[start as usize..offset as usize];

        // Read the firmware.
        let mut start = offset;
        let firmware_length = *&bytes[start as usize] as u8;
        start += 1;
        let offset = start + firmware_length;
        let firmware = &bytes[start as usize..offset as usize];

        // Read the device_id.
        let mut start = offset;
        let device_id_length = *&bytes[start as usize] as u8;
        start += 1;
        let offset = start + device_id_length;
        let device_id = &bytes[start as usize..offset as usize];

        RawSetupConnection::new(
            protocol,
            min_version,
            max_version,
            flags,
            str::from_utf8(endpoint_host)?,
            endpoint_port,
            str::from_utf8(vendor)?,
            str::from_utf8(hardware_version)?,
            str::from_utf8(firmware)?,
            str::from_utf8(device_id)?,
        )
    }

    /// Internal function to frame the RawSetupConnection.
    pub(crate) fn frame(&self) -> Vec<u8> {
        let payload = self.serialize();

        // A size_u24 of the message payload.
        let mut payload_length = (payload.len() as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        serialize!(
            &[0x00, 0x00],                           // empty extension type
            &[MessageTypes::SetupConnection.into()], // msg_type
            &payload_length,
            &payload
        )
    }
}

/// SetupConnectionSuccess is one of the required responses from a
/// Server to a Client when a connection is accepted.
pub struct SetupConnectionSuccess<'a, B>
where
    B: BitFlag,
{
    /// Version proposed by the connecting node as one of the verions supported
    /// by the upstream node. The version will be used during the lifetime of
    /// the connection.
    pub used_version: u16,

    /// Indicates the optional features the server supports in a sub protocol.
    pub flags: &'a [B],
}

impl<'a, B> SetupConnectionSuccess<'a, B>
where
    B: BitFlag,
{
    pub fn new(used_version: u16, flags: &[B]) -> SetupConnectionSuccess<B> {
        SetupConnectionSuccess {
            used_version,
            flags,
        }
    }
}

impl<B> Serializable for SetupConnectionSuccess<'_, B>
where
    B: BitFlag,
{
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let byte_flags = self
            .flags
            .iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |accumulator, byte| (accumulator | byte))
            .to_le_bytes();

        let buffer = serialize!(&self.used_version.to_le_bytes(), &byte_flags);
        Ok(writer.write(&buffer)?)
    }
}

impl<B> Framable for SetupConnectionSuccess<'_, B>
where
    B: BitFlag,
{
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let result = serialize!(
            &[0x00, 0x00],                                  // extention_type
            &[MessageTypes::SetupConnectionSuccess.into()], // msg_type
            &payload_length,
            &payload
        );

        Ok(writer.write(&result)?)
    }
}

/// Contains the error codes for the [SetupConnectionError](struct.SetupConnectionError.html) message.
/// Each error code has a default STR0_255 message.
#[derive(PartialEq)]
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

/// SetupConnectionError is one of the required respones from a Server to a Client
/// when a new connection has failed. The server is required to send this message
/// with an error code before closing the connection.
///
/// If the error is a variant of [UnsupportedFeatureFlags](enum.SetupConnectionErrorCodes.html),
/// the server MUST respond with a all the feature flags that it does NOT support.
///
/// If the flag is 0, then the error is some condition aside from unsupported
/// flags.
pub struct SetupConnectionError<'a, B>
where
    B: BitFlag,
{
    /// Indicates all the flags that the server does NOT support.
    pub flags: &'a [B],

    /// Error code is a predefined STR0_255 error code.
    pub error_code: SetupConnectionErrorCodes,
}

impl<B> SetupConnectionError<'_, B>
where
    B: BitFlag,
{
    /// Constructor for the SetupConnectionError message.
    pub fn new(
        flags: &[B],
        error_code: SetupConnectionErrorCodes,
    ) -> Result<SetupConnectionError<B>> {
        if flags.is_empty() && error_code == SetupConnectionErrorCodes::UnsupportedFeatureFlags {
            return Err(Error::RequirementError(
                "a full set of unsupported flags MUST be returned to the client".into(),
            ));
        }

        Ok(SetupConnectionError {
            flags: &flags,
            error_code,
        })
    }
}

impl<B> Serializable for SetupConnectionError<'_, B>
where
    B: BitFlag,
{
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let byte_flags = self
            .flags
            .iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |accumulator, byte| (accumulator | byte))
            .to_le_bytes();

        let result = serialize!(
            &byte_flags,
            &STR0_255::new(&self.error_code.to_string())?.as_bytes()
        );

        Ok(writer.write(&result)?)
    }
}

impl<B> Framable for SetupConnectionError<'_, B>
where
    B: BitFlag,
{
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let result = serialize!(
            &[0x00, 0x00], // extension_type
            &[MessageTypes::SetupConnectionError.into()],
            &payload_length,
            &payload
        );

        Ok(writer.write(&result)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::Serializable;
    use crate::mining;

    #[test]
    fn serialize_connection_success() {
        let message: SetupConnectionSuccess<'_, mining::SetupConnectionSuccessFlags> =
            SetupConnectionSuccess::new(2, &[]);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [
            0x02, 0x00, // used_version
            0x00, 0x00, 0x00, 0x00, // flags
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_connection_sucess() {
        let message = SetupConnectionSuccess::new(
            2,
            &[mining::SetupConnectionSuccessFlags::RequiresFixedVersion],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [
            0x02, 0x00, // used_version
            0x01, 0x00, 0x00, 0x00, // flags
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_connection_success_all_flags() {
        let message = SetupConnectionSuccess::new(
            2,
            &[
                mining::SetupConnectionSuccessFlags::RequiresFixedVersion,
                mining::SetupConnectionSuccessFlags::RequiresExtendedChannels,
            ],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [
            0x02, 0x00, // used_version
            0x03, 0x00, 0x00, 0x00, // flags
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_connection_success_no_flags() {
        let message: SetupConnectionSuccess<'_, mining::SetupConnectionSuccessFlags> =
            SetupConnectionSuccess::new(2, &[]);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [
            0x02, 0x00, // used_version
            0x00, 0x00, 0x00, 0x00, // flags
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_connection_error() {
        let flags = &[mining::SetupConnectionFlags::RequiresStandardJobs];
        let message =
            SetupConnectionError::new(flags, SetupConnectionErrorCodes::UnsupportedFeatureFlags)
                .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        // Feature flag.
        assert_eq!(buffer[0], 0x01);

        // Length of error code string.
        assert_eq!(buffer[4], 0x19);
    }

    #[test]
    fn serialize_connection_error_empty_flags() {
        let message: Result<SetupConnectionError<'_, mining::SetupConnectionFlags>> =
            SetupConnectionError::new(&[], SetupConnectionErrorCodes::UnsupportedFeatureFlags);

        assert!(message.is_err())
    }

    #[test]
    fn frame_connection_error() {
        let flags = &[mining::SetupConnectionFlags::RequiresStandardJobs];
        let message =
            SetupConnectionError::new(flags, SetupConnectionErrorCodes::UnsupportedFeatureFlags)
                .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        message.frame(&mut buffer).unwrap();

        let expected = [
            0x00, 0x00, // extension_type
            0x03, // msg_type
            0x1e, 0x00, 0x00, // msg_length
            0x01, 0x00, 0x00, 0x00, // flags
            0x19, // length_error_code
            0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66, 0x65,
            0x61, 0x74, 0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73, // error_code
        ];

        assert_eq!(buffer, expected)
    }
}
