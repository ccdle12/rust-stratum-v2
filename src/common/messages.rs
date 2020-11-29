use crate::common::{BitFlag, Framable, Serializable};
use crate::error::{Error, Result};
use crate::job_negotiation;
use crate::mining;
use crate::util::types::{string_to_str0_255, string_to_str0_255_bytes};
use std::fmt;
use std::io;

/// SetupConnection is the first message sent by a client on a new connection.
/// This implementation is a base struct that contains all the common fields
/// for the SetupConnection for each Stratum V2 subprotocol.
pub struct SetupConnection {
    /// Used to indicate the protocol the client wants to use on the new connection.
    ///
    /// Available protocols:
    /// - 0 = Mining Protocol
    /// - 1 = Job Negotiation Protocol
    /// - 2 = Template Distribution Protocol
    /// - 3 = Job Distribution Protocol
    pub protocol: u8,

    /// The minimum protocol version the client supports. (current default: 2)
    pub min_version: u16,

    /// The maxmimum protocol version the client supports. (current default: 2)
    pub max_version: u16,

    /// Flags indicating the optional protocol features the client supports.
    pub flags: Vec<u8>,

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

impl SetupConnection {
    /// Internal constructor for the SetupConnection message. Each subprotcol
    /// has its own public setup connection method that should be called.
    pub(crate) fn new<T: Into<String>>(
        protocol: u8,
        min_version: u16,
        max_version: u16,
        flags: Vec<u8>,
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection> {
        if protocol > 3 {
            return Err(Error::VersionError("invalid protocol request".into()));
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

    /// Constructor for the mining sub protocol setup connection message.
    /// This restricts the caller to only use feature flags from the mining
    /// module.
    pub fn new_mining_connection<T: Into<String>>(
        min_version: u16,
        max_version: u16,
        flags: &[mining::SetupConnectionFlags],
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection> {
        let flags: Vec<u8> = flags.iter().map(|x| x.as_byte()).collect();
        SetupConnection::new(
            0,
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

    /// Constructor for the job negotiation sub protocol setup conenction message.
    /// This restricts the caller to only use feature falgs from the job negotiation
    /// module.
    pub fn new_job_negotiation_connection<T: Into<String>>(
        min_version: u16,
        max_version: u16,
        flags: &[job_negotiation::SetupConnectionFlags],
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection> {
        let flags: Vec<u8> = flags.iter().map(|x| x.as_byte()).collect();
        SetupConnection::new(
            1,
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

/// Implementation of the Serializable trait to serialize the contents
/// of the SetupConnection message to the valid message format.
impl Serializable for SetupConnection {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let mut buffer: Vec<u8> = vec![self.protocol];

        buffer.extend_from_slice(&self.min_version.to_le_bytes());
        buffer.extend_from_slice(&self.max_version.to_le_bytes());

        let byte_flags = (self
            .flags
            .iter()
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

/// Implementation of the Framable trait to build the network message frame
/// specifically for SetupConenction including the serialzed information as the
/// message payload.
impl Framable for SetupConnection {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // TODO: This is repeated for each frame, so maybe create a macro?
        //
        // Default empty channel messsage.
        let channel_msg = &[0x00, 0x00];

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

impl Framable for SetupConnectionSuccess {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // TODO: Need to move to a macro to reduce repetition.
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
pub struct SetupConnectionError<'a, T> {
    flags: &'a [T],
    error_code: SetupConnectionErrorCodes,
}

impl<T> SetupConnectionError<'_, T>
where
    T: BitFlag,
{
    /// Constructor for the SetupConnectionError message.
    pub fn new(flags: &[T], error_code: SetupConnectionErrorCodes) -> SetupConnectionError<T> {
        SetupConnectionError {
            flags: &flags,
            error_code,
        }
    }
}

impl<T> Serializable for SetupConnectionError<'_, T>
where
    T: BitFlag,
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

    #[test]
    fn setup_connection_invalid_min_value() {
        let message = SetupConnection::new(
            0,
            2,
            1,
            vec![0b0001],
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
            0,
            2,
            0,
            vec![0x01],
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
    fn setup_connection_invalid_protocol() {
        let message = SetupConnection::new(
            4,
            2,
            0,
            vec![0x01],
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
    fn mining_setup_connection_init() {
        let message = SetupConnection::new_mining_connection(
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

        assert_eq!(message.protocol, 0);
        assert_eq!(message.min_version, 2);
    }

    #[test]
    fn mining_setup_connection_serialize_0() {
        let message = SetupConnection::new_mining_connection(
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
        let message = SetupConnection::new_mining_connection(
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
        let message = SetupConnection::new_mining_connection(
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
        let message = SetupConnection::new_mining_connection(
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
    fn mining_setup_connection_frame_0() {
        let message = SetupConnection::new_mining_connection(
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
    fn new_job_negotiation_setup_connection_init() {
        let message = SetupConnection::new_job_negotiation_connection(
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
        let message = SetupConnection::new_job_negotiation_connection(
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
        let message = SetupConnection::new_job_negotiation_connection(
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
    fn mining_setup_connection_success_frame_0() {
        let message = SetupConnectionSuccess::new_mining_success(
            2,
            &[mining::SetupConnectionSuccessFlags::RequiresFixedVersion],
        );

        let mut buffer: Vec<u8> = Vec::new();
        // message.serialize(&mut buffer).unwrap();
        message.frame(&mut buffer).unwrap();

        let expected = [
            0x00, 0x00, 0x01, 0x06, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00,
        ];
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
