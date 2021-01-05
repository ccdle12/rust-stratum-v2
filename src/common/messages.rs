use crate::common::types::{MessageTypes, STR0_255};
use crate::common::{BitFlag, Framable, Protocol, Serializable, ToProtocol};
use crate::error::{Error, Result};
use std::fmt;
use std::io;

/// SetupConnection is the first message sent by a client on a new connection.
/// The SetupConnection struct contains all the common fields for the
/// SetupConnection message for each Stratum V2 subprotocol.
pub struct SetupConnection<'a, B>
where
    B: BitFlag + ToProtocol,
{
    /// Used to indicate the protocol the client wants to use on the new connection.
    pub protocol: Protocol,

    /// The minimum protocol version the client supports. (current default: 2)
    pub min_version: u16,

    /// The maxmimum protocol version the client supports. (current default: 2)
    pub max_version: u16,

    /// Flags indicating the optional protocol features the client supports.
    pub flags: &'a [B],

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

impl<'a, B> SetupConnection<'a, B>
where
    B: BitFlag + ToProtocol,
{
    /// Constructor for the SetupConnection message. A specific SetupConnection
    /// can be specified for a sub protocol and an optional channel_id can
    /// be provided to specify the receiver of the message on a particular
    /// channel.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::common::Protocol;
    /// use stratumv2::common::messages::SetupConnection;
    /// use stratumv2::mining;
    /// use stratumv2::job_negotiation;
    /// use stratumv2::util::new_channel_id;
    ///
    ///
    /// let mining_connection = SetupConnection::new(
    ///    Protocol::Mining,
    ///    2,
    ///    2,
    ///    &[
    ///        mining::SetupConnectionFlags::RequiresStandardJobs,
    ///        mining::SetupConnectionFlags::RequiresVersionRolling
    ///     ],
    ///    "0.0.0.0",
    ///    8545,
    ///    "Bitmain",
    ///    "S9i 13.5",
    ///    "braiins-os-2018-09-22-1-hash",
    ///    "some-device-uuid",
    /// );
    ///
    /// let job_negotiation_connection = SetupConnection::new(
    ///    Protocol::JobNegotiation,
    ///    2,
    ///    2,
    ///    &[
    ///        job_negotiation::SetupConnectionFlags::RequiresAsyncJobMining,
    ///     ],
    ///    "0.0.0.0",
    ///    8545,
    ///    "Bitmain",
    ///    "S9i 13.5",
    ///    "braiins-os-2018-09-22-1-hash",
    ///    "some-device-uuid",
    /// );
    ///
    /// assert!(mining_connection.is_ok());
    /// assert!(job_negotiation_connection.is_ok());
    /// ```
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
        let invalid_flag = &flags.into_iter().any(|x| x.as_protocol() != protocol);
        if *invalid_flag {
            return Err(Error::ProtocolMismatchError(
                "flags do not match protocol".into(),
            ));
        }

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
}

/// Implementation of the Serializable trait to serialize the contents
/// of the SetupConnection message to the valid message format.
impl<B> Serializable for SetupConnection<'_, B>
where
    B: BitFlag + ToProtocol,
{
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = Vec::new();

        buffer.push(self.protocol as u8);
        buffer.extend_from_slice(&self.min_version.to_le_bytes());
        buffer.extend_from_slice(&self.max_version.to_le_bytes());

        let byte_flags = self
            .flags
            .iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |accumulator, byte| (accumulator | byte))
            .to_le_bytes();

        buffer.extend_from_slice(&byte_flags);
        buffer.extend_from_slice(&self.endpoint_host.as_bytes());
        buffer.extend_from_slice(&self.endpoint_port.to_le_bytes());
        buffer.extend_from_slice(&self.vendor.as_bytes());
        buffer.extend_from_slice(&self.hardware_version.as_bytes());
        buffer.extend_from_slice(&self.firmware.as_bytes());
        buffer.extend_from_slice(&self.device_id.as_bytes());

        Ok(writer.write(&buffer)?)
    }
}

/// Implementation of the Framable trait to build a network frame for the
/// SetupConnection message.
impl<B> Framable for SetupConnection<'_, B>
where
    B: BitFlag + ToProtocol,
{
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // Currently set to a default empty extension type with MSB NOT set
        // to indicate the message is intended to the direct recipient.
        let extension_type = &[0x00, 0x00];

        // The byte representation of the MessageType for SetupConnection.
        let msg_type = &[MessageTypes::SetupConnection as u8];

        // Serialize SetupConnection as the message payload.
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let mut result = Vec::new();
        result.extend_from_slice(extension_type);
        result.extend_from_slice(msg_type);
        result.extend_from_slice(&payload_length);
        result.extend_from_slice(&payload);

        Ok(writer.write(&result)?)
    }
}

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
        let mut buffer: Vec<u8> = Vec::new();

        buffer.extend_from_slice(&self.used_version.to_le_bytes());

        let byte_flags = self
            .flags
            .iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |accumulator, byte| (accumulator | byte))
            .to_le_bytes();

        buffer.extend_from_slice(&byte_flags);

        Ok(writer.write(&buffer)?)
    }
}

impl<B> Framable for SetupConnectionSuccess<'_, B>
where
    B: BitFlag + ToProtocol,
{
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // TODO: Need to move to a macro or function to reduce repetition
        // in each frame implemenation.
        // Currently set to a default empty extension type with MSB NOT set
        // to indicate the message is intended to the direct recipient.
        let extension_type = &[0x00, 0x00];

        // The byte representation of the MessageType for SetupConnectionSuccess.
        let msg_type = &[MessageTypes::SetupConnectionSuccess as u8];

        // Serialize SetupConnection as the message payload.
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let mut result = Vec::new();
        result.extend_from_slice(extension_type);
        result.extend_from_slice(msg_type);
        result.extend_from_slice(&payload_length);
        result.extend_from_slice(&payload);

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
        let mut buffer: Vec<u8> = Vec::new();

        let byte_flags = self
            .flags
            .iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |accumulator, byte| (accumulator | byte))
            .to_le_bytes();

        buffer.extend_from_slice(&byte_flags);
        buffer.extend_from_slice(&STR0_255::new(&self.error_code.to_string())?.as_bytes());

        Ok(writer.write(&buffer)?)
    }
}

#[cfg(test)]
mod setup_connection_tests {
    use super::*;
    use crate::common::Serializable;
    use crate::mining;

    #[test]
    fn init_setup_connection() {
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
    fn setup_connection_invalid_flag_and_protocol() {
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
            1,
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
    fn setup_connection_empty_vendor() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_err())
    }

    #[test]
    fn setup_connection_empty_firmware() {
        let message = SetupConnection::new(
            Protocol::Mining,
            2,
            2,
            &[mining::SetupConnectionFlags::RequiresStandardJobs],
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "",
            "some-uuid",
        );

        assert!(message.is_err())
    }

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
}

#[cfg(test)]
mod mining_connection_tests {
    use super::*;
    use crate::common::{Framable, Serializable};
    use crate::mining;

    #[test]
    fn serialize_mining_connection() {
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
            0x00, // protocol
            0x02, 0x00, // min_version
            0x02, 0x00, // max_version
            0x01, 0x00, 0x00, 0x00, // flags
            0x07, // length_endpoint_host
            0x30, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x30, // endpoint_host
            0x61, 0x21, // endpoint_port
            0x07, // length_vendor
            0x42, 0x69, 0x74, 0x6d, 0x61, 0x69, 0x6e, // vendor
            0x08, // length_hardware_version
            0x53, 0x39, 0x69, 0x20, 0x31, 0x33, 0x2e, 0x35, // hardware_version
            0x1c, // length_firmware
            0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x2d, 0x6f, 0x73, 0x2d, 0x32, 0x30, 0x31,
            0x38, 0x2d, 0x30, 0x39, 0x2d, 0x32, 0x32, 0x2d, 0x31, 0x2d, 0x68, 0x61, 0x73,
            0x68, // firmware
            0x09, // length_device_id
            0x73, 0x6f, 0x6d, 0x65, 0x2d, 0x75, 0x75, 0x69, 0x64, // device_id
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_no_flags() {
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
    fn serialize_multiple_flags() {
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
    fn serialilze_all_flags() {
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
    fn frame_setup_connection() {
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

        let size = message.frame(&mut buffer).unwrap();
        assert_eq!(size, 81);

        let expected = [
            0x00, 0x00, // extension_type
            0x00, // msg_type
            0x4b, 0x00, 0x00, // msg_length
            0x00, // protocol
            0x02, 0x00, // min_version
            0x02, 0x00, // max_version
            0x01, 0x00, 0x00, 0x00, // flags
            0x07, // length_endpoint_host
            0x30, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x30, // endpoint_host
            0x61, 0x21, // endpoint_port
            0x07, // length_vendor
            0x42, 0x69, 0x74, 0x6d, 0x61, 0x69, 0x6e, // vendor
            0x08, // length_hardware_version
            0x53, 0x39, 0x69, 0x20, 0x31, 0x33, 0x2e, 0x35, // hardware_version
            0x1c, // length_firmeware
            0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x2d, 0x6f, 0x73, 0x2d, 0x32, 0x30, 0x31,
            0x38, 0x2d, 0x30, 0x39, 0x2d, 0x32, 0x32, 0x2d, 0x31, 0x2d, 0x68, 0x61, 0x73,
            0x68, // firmeware
            0x09, // length_device_id
            0x73, 0x6f, 0x6d, 0x65, 0x2d, 0x75, 0x75, 0x69, 0x64, // device_id
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

#[cfg(test)]
mod job_negotiation_connection_tests {
    use super::*;
    use crate::common::Serializable;
    use crate::job_negotiation;

    #[test]
    fn init_job_negotiation_connection() {
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
    fn serialize_job_negotiation() {
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
    fn serialize_job_negotiation_no_flags() {
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
}
