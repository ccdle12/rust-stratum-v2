use crate::common::types::{MessageTypes, STR0_255};
use crate::common::{BitFlag, Framable, Serializable, ToProtocol};
use crate::error::{Error, Result};
use std::{fmt, io};

/// SetupConnectionSuccess is one of the required responses from a
/// Server to a Client when a connection is accepted.
pub struct SetupConnectionSuccess<'a, B>
where
    B: BitFlag + ToProtocol,
{
    /// Version proposed by the connecting node as one of the verions supported
    /// by the upstream node. The version will be used during the lifetime of
    /// the connection.
    pub used_version: u16,

    /// Indicates the optional features the server supports.
    pub flags: &'a [B],
}

impl<'a, B> SetupConnectionSuccess<'a, B>
where
    B: BitFlag + ToProtocol,
{
    /// Constructor for the SetupConnectionSuccess message.
    pub fn new(used_version: u16, flags: &[B]) -> SetupConnectionSuccess<B> {
        SetupConnectionSuccess {
            used_version,
            flags,
        }
    }
}

impl<B> Serializable for SetupConnectionSuccess<'_, B>
where
    B: BitFlag + ToProtocol,
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
    B: BitFlag + ToProtocol,
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

/// SetupConnectionError is one of the required respones from a server to client
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
    B: BitFlag + ToProtocol,
{
    /// Indicates all the flags that the server does NOT support.
    pub flags: &'a [B],

    /// Error code is a predefined STR0_255 error code.
    pub error_code: SetupConnectionErrorCodes,
}

impl<B> SetupConnectionError<'_, B>
where
    B: BitFlag + ToProtocol,
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
    B: BitFlag + ToProtocol,
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
    B: BitFlag + ToProtocol,
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
mod tests {
    use super::*;
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
    fn frame_connection_success() {
        let message = SetupConnectionSuccess::new(
            2,
            &[mining::SetupConnectionSuccessFlags::RequiresFixedVersion],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.frame(&mut buffer).unwrap();

        let expected = [
            0x00, 0x00, // extension_type
            0x01, // msg_type
            0x06, 0x00, 0x00, // msg_length
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
}
