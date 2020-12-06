use crate::common::{BitFlag, Framable, Serializable, ToProtocol};
use crate::error::{Error, Result};
use crate::util::types::{string_to_str0_255, string_to_str0_255_bytes};
use std::fmt;
use std::io;

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
/// Protocol is an enum representing each sub protocol of Stratum V2.
pub enum Protocol {
    /// Mining is the main and only required sub protocol in Stratum V2.
    Mining = 0,

    /// JobNegotiation is a protocol for intermediate nodes to broker
    /// the terms of a connection between downstream nodes and upstream nodes.
    JobNegotiation = 1,

    /// TemplateDistribution is a protocol for getting the next block from the
    /// Bitcoin RPC. This protocol is intented to replace getblocktemplate.
    TemplateDistribution = 2,

    /// JobDistribution is a protocol for passing newly-negotiated work from the
    /// Job Negotiator to proxies or mining devices. If miners aren't choosing
    /// their transaction sets, then jobs will be distributed from pools directly
    /// to proxies/mining devices.
    JobDistribution = 3,
}

/// SetupConnection is the first message sent by a client on a new connection.
/// The SetupConnection struct contains all the common fields for the
/// SetupConnection message for each Stratum V2 subprotocol.
pub struct SetupConnection<'a, B>
where
    B: BitFlag + ToProtocol,
{
    /// Optional Parameter to indicate if the message is intended for a certain
    /// channel.
    pub channel_id: Option<u32>,

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
    /// Constructor for the SetupConnection message. A specific SetupConnection
    /// can be specified for a sub protocol.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::common::{Protocol, SetupConnection};
    /// use stratumv2::mining;
    /// use stratumv2::job_negotiation;
    /// use stratumv2::util::new_channel_id;
    ///
    ///
    /// let channel_id = new_channel_id();
    ///
    /// let mining_connection = SetupConnection::new(
    ///    Some(channel_id),
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
    ///    None,
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
        channel_id: Option<u32>,
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
            channel_id,
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

/// Implementation of the Serializable trait to serialize the contents
/// of the SetupConnection message to the valid message format.
impl<B> Serializable for SetupConnection<'_, B>
where
    B: BitFlag + ToProtocol,
{
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = Vec::new();

        if self.channel_id.is_some() {
            buffer.extend_from_slice(&self.channel_id.unwrap().to_le_bytes());
        }

        buffer.push(self.protocol as u8);
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

/// Implementation of the Framable trait to build a network frame for the
/// SetupConnection message.
impl<B> Framable for SetupConnection<'_, B>
where
    B: BitFlag + ToProtocol,
{
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let channel_msg = match self.channel_id {
            Some(_) => &[0x80, 0x00],
            None => &[0x00, 0x00],
        };

        // Message type of SetupConnection is always 0x00.
        let msg_type = &[0x00];

        // Serialize SetupConnection as the message payload.
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let mut result = Vec::new();
        result.extend_from_slice(channel_msg);
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
    /// Version proposed by the connecting node that the upstream node (Server?)
    /// supports. The version will be used during the lifetime of the connection.
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

        let byte_flags = (self
            .flags
            .iter()
            .map(|x| x.as_byte())
            .fold(0, |accumulator, byte| (accumulator | byte)) as u32)
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
        // Default empty channel messsage.
        let channel_msg = &[0x00, 0x00];

        // Message type of SetupConnectionSuccess is always 0x01.
        let msg_type = &[0x01];

        // Serialize SetupConnection as the message payload.
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let mut result = Vec::new();
        result.extend_from_slice(channel_msg);
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
mod setup_connection_tests {
    use super::*;
    use crate::common::Serializable;
    use crate::mining;

    #[test]
    fn setup_connection_init() {
        let message = SetupConnection::new(
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
            None,
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
    fn setup_connection_success() {
        let message: SetupConnectionSuccess<'_, mining::SetupConnectionSuccessFlags> =
            SetupConnectionSuccess::new(2, &[]);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }
}

#[cfg(test)]
mod mining_connection_tests {
    use super::*;
    use crate::common::{Framable, Serializable};
    use crate::mining;
    use crate::util::new_channel_id;
    use std::convert::TryInto;

    #[test]
    fn serialize_no_channel_id() {
        let message = SetupConnection::new(
            None,
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
    fn serialize_no_flags() {
        let message: SetupConnection<'_, mining::SetupConnectionFlags> = SetupConnection::new(
            None,
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
            None,
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
            None,
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
    fn serialize_with_channel_id() {
        let channel_id = new_channel_id();
        let message = SetupConnection::new(
            Some(channel_id),
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

        // The message should be an additional 4 bytes longer due to the
        // specified channel_id.
        assert_eq!(size, 79);

        // Assert the first 4 deserialized bytes match the assigned channel_id.
        assert_eq!(
            u32::from_le_bytes(buffer[0..4].try_into().unwrap()),
            channel_id
        );
    }

    #[test]
    fn connection_frame_without_channel_id() {
        let message = SetupConnection::new(
            None,
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
            0x00, 0x00, 0x00, 0x4b, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x07, 0x30, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x30, 0x61, 0x21, 0x07, 0x42, 0x69,
            0x74, 0x6d, 0x61, 0x69, 0x6e, 0x08, 0x53, 0x39, 0x69, 0x20, 0x31, 0x33, 0x2e, 0x35,
            0x1c, 0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x2d, 0x6f, 0x73, 0x2d, 0x32, 0x30,
            0x31, 0x38, 0x2d, 0x30, 0x39, 0x2d, 0x32, 0x32, 0x2d, 0x31, 0x2d, 0x68, 0x61, 0x73,
            0x68, 0x09, 0x73, 0x6f, 0x6d, 0x65, 0x2d, 0x75, 0x75, 0x69, 0x64,
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn connection_frame_with_channel_id() {
        let message = SetupConnection::new(
            Some(32),
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

        // Assert the entire frame has an additional 4 bytes for the channel_id
        // in the payload.
        assert_eq!(size, 85);

        let expected = [
            0x80, 0x00, 0x00, 0x4f, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x07, 0x30, 0x2e, 0x30, 0x2e, 0x30, 0x2e, 0x30, 0x61,
            0x21, 0x07, 0x42, 0x69, 0x74, 0x6d, 0x61, 0x69, 0x6e, 0x08, 0x53, 0x39, 0x69, 0x20,
            0x31, 0x33, 0x2e, 0x35, 0x1c, 0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x2d, 0x6f,
            0x73, 0x2d, 0x32, 0x30, 0x31, 0x38, 0x2d, 0x30, 0x39, 0x2d, 0x32, 0x32, 0x2d, 0x31,
            0x2d, 0x68, 0x61, 0x73, 0x68, 0x09, 0x73, 0x6f, 0x6d, 0x65, 0x2d, 0x75, 0x75, 0x69,
            0x64,
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_success_0() {
        let message = SetupConnectionSuccess::new(
            2,
            &[mining::SetupConnectionSuccessFlags::RequiresFixedVersion],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_success_frame_0() {
        let message = SetupConnectionSuccess::new(
            2,
            &[mining::SetupConnectionSuccessFlags::RequiresFixedVersion],
        );

        let mut buffer: Vec<u8> = Vec::new();
        message.frame(&mut buffer).unwrap();

        let expected = [
            0x00, 0x00, 0x01, 0x06, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn mining_setup_connection_success_1() {
        let message = SetupConnectionSuccess::new(
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
        let message: SetupConnectionSuccess<'_, mining::SetupConnectionSuccessFlags> =
            SetupConnectionSuccess::new(2, &[]);

        let mut buffer: Vec<u8> = Vec::new();
        message.serialize(&mut buffer).unwrap();

        let expected = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(buffer, expected);
    }
}

#[cfg(test)]
mod job_negotiation_connection_tests {
    use super::*;
    use crate::common::Serializable;
    use crate::{job_negotiation, mining};

    #[test]
    fn new_job_negotiation_setup_connection_init() {
        let message = SetupConnection::new(
            None,
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
            None,
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
                None,
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
    fn setup_connection_error_serialize_0() {
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
    fn setup_connection_invalid_empty_flags() {
        let message: Result<SetupConnectionError<'_, mining::SetupConnectionFlags>> =
            SetupConnectionError::new(&[], SetupConnectionErrorCodes::UnsupportedFeatureFlags);

        assert!(message.is_err())
    }
}
