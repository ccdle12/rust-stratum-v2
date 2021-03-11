use crate::common::SetupConnectionErrorCodes;
use crate::error::{Error, Result};
use crate::mining::{SetupConnectionFlags, SetupConnectionSuccessFlags};
use crate::types::{MessageTypes, B0_32, STR0_255, STR0_32, U256};
use crate::util::ByteParser;
use crate::{BitFlag, Deserializable, Frameable, Protocol, Serializable};
use std::borrow::Cow;
use std::fmt;
use std::{io, str};

// Implementation of the SetupConenction, SetupConnectionSuccess and SetupConnectionError
// messages for the Mining Protocol.
impl_setup_connection!(Protocol::Mining, SetupConnectionFlags);
impl_setup_connection_success!(SetupConnectionSuccessFlags);
impl_setup_connection_error!(SetupConnectionFlags);

/// OpenStandardMiningChannel is a message sent by the Client to the Server
/// after a [SetupConnection.Success](struct.SetupConnectionSuccess.html) is
/// sent from the Server. This message is used to request opening a standard
/// channel to the upstream server. A standard mining channel indicates `header-only`
/// mining.
pub struct OpenStandardMiningChannel {
    /// A Client-specified unique identifier across all client connections.
    /// The request_id is not interpreted by the Server.
    pub request_id: u32,

    /// A sequence of bytes that identifies the node to the Server, e.g.
    /// "braiintest.worker1".
    pub user_identity: STR0_255,

    /// The expected [h/s] (hash rate/per second) of the
    /// device or the cumulative on the channel if multiple devices are connected
    /// downstream. Proxies MUST send 0.0f when there are no mining devices
    /// connected yet.
    pub nominal_hash_rate: f32,

    /// The Maximum Target that can be acceptd by the connected device or
    /// multiple devices downstream. The Server MUST accept the maximum
    /// target or respond by sending a
    /// [OpenMiningChannel.Error](struct.OpenMiningChannelError.html)
    pub max_target: U256,
}

impl OpenStandardMiningChannel {
    pub fn new<T: Into<String>>(
        request_id: u32,
        user_identity: T,
        nominal_hash_rate: f32,
        max_target: U256,
    ) -> Result<OpenStandardMiningChannel> {
        Ok(OpenStandardMiningChannel {
            request_id,
            user_identity: STR0_255::new(user_identity)?,
            nominal_hash_rate,
            max_target,
        })
    }
}

impl Serializable for OpenStandardMiningChannel {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = serialize_slices!(
            &self.request_id.to_le_bytes(),
            &self.user_identity.as_bytes(),
            &self.nominal_hash_rate.to_le_bytes(),
            &self.max_target
        );

        Ok(writer.write(&buffer)?)
    }
}

impl Deserializable for OpenStandardMiningChannel {
    fn deserialize(bytes: &[u8]) -> Result<OpenStandardMiningChannel> {
        let mut parser = ByteParser::new(bytes, 0);

        let request_id = parser.next_by(4)?;
        let user_identity_length = parser.next_by(1)?[0] as usize;
        let user_identity = parser.next_by(user_identity_length)?;
        let nominal_hash_rate = parser.next_by(4)?;
        let max_target = parser.next_by(32)?;

        OpenStandardMiningChannel::new(
            u32::from_le_bytes(request_id.try_into()?),
            str::from_utf8(user_identity)?,
            f32::from_le_bytes(nominal_hash_rate.try_into()?),
            max_target.try_into()?,
        )
    }
}

impl_frameable_trait!(
    OpenStandardMiningChannel,
    MessageTypes::OpenStandardMiningChannel,
    false
);

/// OpenStandardMiningChannelSuccess is a message sent by the Server to the Client
/// in response to opening a standard mining channel if succesful.
pub struct OpenStandardMiningChannelSuccess {
    /// The request_id received in the OpenStandardMiningChannel message. This
    /// is returned to the Client so that they can pair the responses with the
    /// initial request.
    request_id: u32,

    /// Assigned by the Server to uniquely identify the channel, the id is stable
    /// for the whole lifetime of the connection.
    channel_id: u32,

    /// The initial target difficulty target for the mining channel.
    target: U256,

    // TODO: I don't understand the purpose of the extranonce_prefix.
    extranonce_prefix: B0_32,

    /// Group channel that the channel belongs to.
    group_channel_id: u32,
}

impl OpenStandardMiningChannelSuccess {
    pub fn new<T: Into<Vec<u8>>>(
        request_id: u32,
        channel_id: u32,
        target: U256,
        extranonce_prefix: T,
        group_channel_id: u32,
    ) -> Result<OpenStandardMiningChannelSuccess> {
        Ok(OpenStandardMiningChannelSuccess {
            request_id,
            channel_id,
            target,
            extranonce_prefix: B0_32::new(extranonce_prefix.into())?,
            group_channel_id,
        })
    }
}

impl Serializable for OpenStandardMiningChannelSuccess {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = serialize_slices!(
            &self.request_id.to_le_bytes(),
            &self.channel_id.to_le_bytes(),
            &self.target,
            &self.extranonce_prefix.as_bytes(),
            &self.group_channel_id.to_le_bytes()
        );

        Ok(writer.write(&buffer)?)
    }
}

impl Deserializable for OpenStandardMiningChannelSuccess {
    fn deserialize(bytes: &[u8]) -> Result<OpenStandardMiningChannelSuccess> {
        let mut parser = ByteParser::new(bytes, 0);

        let request_id = parser.next_by(4)?;
        let channel_id = parser.next_by(4)?;
        let target = parser.next_by(32)?;
        let extranonce_length = parser.next_by(1)?[0] as usize;
        let extranonce_bytes = parser.next_by(extranonce_length)?;
        let group_channel_id = parser.next_by(4)?;

        OpenStandardMiningChannelSuccess::new(
            u32::from_le_bytes(request_id.try_into()?),
            u32::from_le_bytes(channel_id.try_into()?),
            target.try_into()?,
            extranonce_bytes.to_vec(),
            u32::from_le_bytes(group_channel_id.try_into()?),
        )
    }
}

impl_frameable_trait!(
    OpenStandardMiningChannelSuccess,
    MessageTypes::OpenStandardMiningChannelSuccess,
    false
);

// Implementation of the OpenMiningChannelError messages for Standard and Extended
// mining.
impl_open_mining_channel_error!(
    OpenStandardMiningChannelError,
    MessageTypes::OpenStandardMiningChannelError
);

impl_open_mining_channel_error!(
    OpenExtendedMiningChannelError,
    MessageTypes::OpenExtendedMiningChannelError
);

/// Contains the error codes for the [OpenMiningChannelError](struct.OpenMiningChannelError.html)
/// message. Each error code is serialized according to constraints of a STR0_32.
#[derive(Debug, PartialEq)]
pub enum OpenMiningChannelErrorCodes {
    UnknownUser,
    MaxTargetOutOfRange,
}

impl_error_codes_enum!(
    OpenMiningChannelErrorCodes,
    OpenMiningChannelErrorCodes::UnknownUser => "unknown-user",
    OpenMiningChannelErrorCodes::MaxTargetOutOfRange => "max-target-out-of-range"

);

#[cfg(test)]
mod setup_connection_tests {
    use super::*;
    use crate::util::{frame, serialize};

    #[test]
    fn init_setup_connection() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
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
    fn setup_connection_invalid_min_value() {
        let message = SetupConnection::new(
            1,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
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
            2,
            0,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
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
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
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
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
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
    fn serialize_setup_connection() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let buffer = serialize(message).unwrap();
        assert_eq!(buffer.len(), 75);

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

        // Sanity check - deserializing the struct does not return errors.
        assert!(SetupConnection::deserialize(&buffer).is_ok());
    }

    #[test]
    fn serialize_no_flags() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let buffer = serialize(message).unwrap();
        assert_eq!(buffer.len(), 75);

        // Expect the feature flag to have no set flags (0x00).
        assert_eq!(buffer[5], 0x00);
    }

    #[test]
    fn serialize_multiple_flags() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[
                SetupConnectionFlags::RequiresStandardJobs,
                SetupConnectionFlags::RequiresVersionRolling,
            ]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let buffer = serialize(message).unwrap();

        assert_eq!(buffer.len(), 75);
        assert_eq!(buffer[5], 0x05);
    }

    #[test]
    fn serialilze_all_flags() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[
                SetupConnectionFlags::RequiresStandardJobs,
                SetupConnectionFlags::RequiresWorkSelection,
                SetupConnectionFlags::RequiresVersionRolling,
            ]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let buffer = serialize(message).unwrap();

        assert_eq!(buffer.len(), 75);
        assert_eq!(buffer[5], 0x07);
    }

    #[test]
    fn deserialize_setup_connection() {
        let input = [
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

        let message = SetupConnection::deserialize(&input).unwrap();
        assert_eq!(message.min_version, 2);
        assert_eq!(message.max_version, 2);
        assert_eq!(message.flags[0], SetupConnectionFlags::RequiresStandardJobs);
        assert_eq!(message.endpoint_host, "0.0.0.0".to_string());
        assert_eq!(message.endpoint_port, 8545);
        assert_eq!(message.vendor, "Bitmain".to_string());
        assert_eq!(message.hardware_version, "S9i 13.5".to_string());
        assert_eq!(message.firmware, "braiins-os-2018-09-22-1-hash".to_string());
        assert_eq!(message.device_id, "some-uuid".to_string());
    }

    #[test]
    fn deserialize_malformed_setup_connection() {
        // Empty message.
        let input = [];
        assert!(SetupConnection::deserialize(&input).is_err());

        // Unknown protocol.
        let input = [0xAF];
        assert!(SetupConnection::deserialize(&input).is_err());

        let input = [
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

        // Append each byte from input to a new vector of bytes. This should
        // return errors each time on deserialization since the message is
        // malformed because the message is incomplete.
        let mut output = vec![];
        for i in input.iter() {
            assert!(SetupConnection::deserialize(&output).is_err());
            output.push(*i);
        }

        // Now that the vector of bytes contains the full message, deserializing
        // should return ok.
        assert!(SetupConnection::deserialize(&output).is_ok());
    }

    #[test]
    fn frame_setup_connection() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let buffer = frame(message).unwrap();
        assert_eq!(buffer.len(), 81);

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
}

#[cfg(test)]
mod open_standard_mining_tests {
    use super::*;
    use crate::util::{frame, new_channel_id, serialize};

    #[test]
    fn open_standard_mining_channel() {
        // TODO: Substitute the nominal hash rate with something more accurate.
        let target = [0u8; 32];

        let message =
            OpenStandardMiningChannel::new(1, "braiinstest.worker1".to_string(), 12.3, target)
                .unwrap();

        assert_eq!(message.request_id, 1);
        assert_eq!(message.user_identity, "braiinstest.worker1".to_string());
        assert_eq!(message.nominal_hash_rate, 12.3);
        assert_eq!(message.max_target.len(), 32);
    }

    #[test]
    fn serialize_open_standard_mining_channel() {
        let expected = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x13, // length_user_identity
            0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x77, 0x6f,
            0x72, 0x6b, 0x65, 0x72, 0x31, // user_identity
            0xcd, 0xcc, 0x44, 0x41, // nominal_hash_rate
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // max_target
        ];

        let message =
            OpenStandardMiningChannel::new(1, "braiinstest.worker1".to_string(), 12.3, [0u8; 32])
                .unwrap();

        assert_eq!(serialize(message).unwrap(), expected);
    }

    #[test]
    fn deserialize_open_standard_mining_channel() {
        let input = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x13, // length_user_identity
            0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x77, 0x6f,
            0x72, 0x6b, 0x65, 0x72, 0x31, // user_identity
            0xcd, 0xcc, 0x44, 0x41, // nominal_hash_rate
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // max_target
        ];

        let message = OpenStandardMiningChannel::deserialize(&input).unwrap();
        assert_eq!(message.request_id, 1);
        assert_eq!(message.user_identity, "braiinstest.worker1".to_string());
        assert_eq!(message.nominal_hash_rate, 12.3);
        assert_eq!(message.max_target, [0u8; 32]);
    }

    #[test]
    fn open_standard_mining_sucess() {
        let extranonce_prefix = [0x00, 0x00];
        let channel_id = new_channel_id();
        let message =
            OpenStandardMiningChannelSuccess::new(1, channel_id, [0u8; 32], extranonce_prefix, 1)
                .unwrap();

        assert_eq!(message.request_id, 1);
        assert_eq!(message.channel_id, channel_id);
        assert_eq!(message.target, [0u8; 32]);
        assert_eq!(message.extranonce_prefix, extranonce_prefix.to_vec());
        assert_eq!(message.group_channel_id, 1);
    }

    #[test]
    fn serialize_open_standard_mining_success() {
        let expected = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x01, 0x00, 0x00, 0x00, // channel_id
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // target
            0x02, // length extranonce_prefix
            0x00, 0x00, // extranonce_prefix
            0x01, 0x00, 0x00, 0x00, // request_id
        ];

        let message =
            OpenStandardMiningChannelSuccess::new(1, 1, [0u8; 32], vec![0x00, 0x00], 1).unwrap();

        let buffer = serialize(message).unwrap();

        assert_eq!(buffer, expected);
    }

    #[test]
    fn deserialize_open_standard_mining_success() {
        let input = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x01, 0x00, 0x00, 0x00, // channel_id
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // target
            0x02, // length extranonce_prefix
            0x00, 0x00, // extranonce_prefix
            0x01, 0x00, 0x00, 0x00, // request_id
        ];

        let message = OpenStandardMiningChannelSuccess::deserialize(&input).unwrap();
        assert_eq!(message.request_id, 1);
        assert_eq!(message.channel_id, 1);
        assert_eq!(message.target, [0u8; 32]);
        assert_eq!(message.extranonce_prefix, vec![0x00, 0x00]);
        assert_eq!(message.group_channel_id, 1);
    }

    #[test]
    fn frame_open_standard_mining() {
        let target = [0u8; 32];

        let message =
            OpenStandardMiningChannel::new(1, "braiinstest.worker1".to_string(), 12.3, target)
                .unwrap();

        let buffer = frame(message).unwrap();

        let expected = [
            0x00, 0x00, // extension_type
            0x10, // msg_type
            0x3c, 0x00, 0x00, // msg_length
            0x01, 0x00, 0x00, 0x00, // request_id
            0x13, // length_user_identity
            0x62, 0x72, 0x61, 0x69, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x77, 0x6f,
            0x72, 0x6b, 0x65, 0x72, 0x31, // user_identity
            0xcd, 0xcc, 0x44, 0x41, // nominal_hash_rate
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // max_target
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn frame_open_standard_mining_success() {
        let message =
            OpenStandardMiningChannelSuccess::new(1, 1, [0u8; 32], vec![0x00, 0x00], 1).unwrap();

        let expected = [
            0x00, 0x00, // extension_type
            0x11, // msg_type
            0x2f, 0x00, 0x00, // msg_length
            0x01, 0x00, 0x00, 0x00, // request_id
            0x01, 0x00, 0x00, 0x00, // channel_id
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // target
            0x02, // length extranonce_prefix
            0x00, 0x00, // extranonce_prefix
            0x01, 0x00, 0x00, 0x00, // request_id
        ];

        let buffer = frame(message).unwrap();
        assert_eq!(buffer, expected);
    }

    #[test]
    fn open_standard_mining_channel_error() {
        let message =
            OpenStandardMiningChannelError::new(1, OpenMiningChannelErrorCodes::UnknownUser);

        assert_eq!(message.request_id, 1);
        assert_eq!(message.error_code, OpenMiningChannelErrorCodes::UnknownUser);
    }

    #[test]
    fn open_standard_mining_channel_error_serialize() {
        let message =
            OpenStandardMiningChannelError::new(1, OpenMiningChannelErrorCodes::UnknownUser);

        let bytes = serialize(message).unwrap();
        let expected = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x0c, // error_code_length
            0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x2d, 0x75, 0x73, 0x65,
            0x72, // error_code
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn open_standard_mining_channel_error_deserialize() {
        let input = [
            0x01, 0x00, 0x00, 0x00, // request_id
            0x0c, // error_code_length
            0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x2d, 0x75, 0x73, 0x65,
            0x72, // error_code
        ];

        let message = OpenStandardMiningChannelError::deserialize(&input).unwrap();

        assert_eq!(message.request_id, 1);
        assert_eq!(message.error_code, OpenMiningChannelErrorCodes::UnknownUser);
    }

    #[test]
    fn frame_open_standard_mining_channel_error() {
        let message =
            OpenStandardMiningChannelError::new(1, OpenMiningChannelErrorCodes::UnknownUser);

        let expected = [
            0x00, 0x00, // extension_type
            0x12, // msg_type
            0x11, 0x00, 0x00, // msg_length
            0x01, 0x00, 0x00, 0x00, // request_id
            0x0c, // error_code_length
            0x75, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x2d, 0x75, 0x73, 0x65,
            0x72, // error_code
        ];

        let buffer = frame(message).unwrap();
        assert_eq!(buffer, expected);
    }

    #[test]
    fn frame_open_extended_mining_channel_error() {
        let message =
            OpenExtendedMiningChannelError::new(1, OpenMiningChannelErrorCodes::UnknownUser);

        let buffer = frame(message).unwrap();
        assert_eq!(buffer[2], 0x15);
    }
}

#[cfg(test)]
mod connection_success_tests {
    use super::*;
    use crate::util::{frame, serialize};

    #[test]
    fn serialize_connection_success() {
        let message = SetupConnectionSuccess::new(2, Cow::Borrowed(&[]));

        let buffer = serialize(message).unwrap();

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
            Cow::Borrowed(&[SetupConnectionSuccessFlags::RequiresFixedVersion]),
        );

        let buffer = serialize(message).unwrap();

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
            Cow::Borrowed(&[SetupConnectionSuccessFlags::RequiresFixedVersion]),
        );

        let buffer = frame(message).unwrap();

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
            Cow::Borrowed(&[
                SetupConnectionSuccessFlags::RequiresFixedVersion,
                SetupConnectionSuccessFlags::RequiresExtendedChannels,
            ]),
        );

        let buffer = serialize(message).unwrap();

        let expected = [
            0x02, 0x00, // used_version
            0x03, 0x00, 0x00, 0x00, // flags
        ];
        assert_eq!(buffer, expected);
    }

    #[test]
    fn serialize_connection_success_no_flags() {
        let message = SetupConnectionSuccess::new(2, Cow::Borrowed(&[]));

        let buffer = serialize(message).unwrap();

        let expected = [
            0x02, 0x00, // used_version
            0x00, 0x00, 0x00, 0x00, // flags
        ];
        assert_eq!(buffer, expected);
    }
}

#[cfg(test)]
mod connection_error_tests {
    use super::*;
    use crate::util::{frame, serialize};

    #[test]
    fn serialize_connection_error() {
        let message = SetupConnectionError::new(
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
            SetupConnectionErrorCodes::UnsupportedFeatureFlags,
        )
        .unwrap();

        let buffer = serialize(message).unwrap();

        // Feature flag.
        assert_eq!(buffer[0], 0x01);

        // Length of error code string.
        assert_eq!(buffer[4], 0x19);
    }

    #[test]
    fn serialize_connection_error_empty_flags() {
        let message = SetupConnectionError::new(
            Cow::Borrowed(&[]),
            SetupConnectionErrorCodes::UnsupportedFeatureFlags,
        );

        assert!(message.is_err())
    }

    #[test]
    fn frame_connection_error() {
        let message = SetupConnectionError::new(
            Cow::Borrowed(&[SetupConnectionFlags::RequiresStandardJobs]),
            SetupConnectionErrorCodes::UnsupportedFeatureFlags,
        )
        .unwrap();

        let buffer = frame(message).unwrap();

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
    fn deserialize_connection_error() {
        let message = [
            0x01, 0x00, 0x00, 0x00, // flags
            0x19, // length_error_code
            0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66, 0x65,
            0x61, 0x74, 0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73, // error_code
        ];

        let conn_error = SetupConnectionError::deserialize(&message).unwrap();
        assert_eq!(
            conn_error.flags[0],
            SetupConnectionFlags::RequiresStandardJobs
        );
        assert_eq!(
            conn_error.error_code,
            SetupConnectionErrorCodes::UnsupportedFeatureFlags
        );
    }

    #[test]
    fn deserialize_malformed_connection_error() {
        // Empty message.
        let input = [];
        assert!(SetupConnection::deserialize(&input).is_err());

        let input = [
            0x01, 0x00, 0x00, 0x00, // flags
            0x19, // length_error_code
            0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66, 0x65,
            0x61, 0x74, 0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73, // error_code
        ];

        let mut output = vec![];
        for i in input.iter() {
            assert!(SetupConnectionError::deserialize(&output).is_err());
            output.push(*i);
        }

        assert!(SetupConnectionError::deserialize(&output).is_ok());

        // Incorrect length_error_code.
        let input = [
            0x01, 0x00, 0x00, 0x00, // flags
            0xFF, // length_error_code
            0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66, 0x65,
            0x61, 0x74, 0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73, // error_code
        ];
        assert!(SetupConnectionError::deserialize(&input).is_err());

        // Invalid flags.
        let input = [
            0xff, 0xff, 0xff, 0xff, // flags
            0xff, // length_error_code
            0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66, 0x65,
            0x61, 0x74, 0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73, // error_code
        ];
        assert!(SetupConnectionError::deserialize(&input).is_err());

        // Invalid error code.
        let input = [
            0x01, 0x00, 0x00, 0x00, // flags
            0x19, // length_error_code
            0xff, 0xff, 0xff, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66, 0x65,
            0xff, 0xff, 0xff, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73, // error_code
        ];
        assert!(SetupConnectionError::deserialize(&input).is_err());
    }
}
