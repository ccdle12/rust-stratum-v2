use crate::common::messages::InternalSetupConnection;
use crate::common::types::U256;
use crate::common::types::{MessageTypes, STR0_255};
use crate::common::{BitFlag, Framable, Protocol, Serializable, ToProtocol};
use crate::error::{Error, Result};
use crate::mining;
use std::{fmt, io, str};

// TODO: Turn this into a macro
pub struct SetupConnection {
    internal: InternalSetupConnection,
}

impl SetupConnection {
    pub fn new<T: Into<String>>(
        min_version: u16,
        max_version: u16,
        flags: Vec<mining::SetupConnectionFlags>,
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection> {
        let flags = flags
            .into_iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |acc, byte| (acc | byte));

        let internal = InternalSetupConnection::new(
            Protocol::Mining,
            min_version,
            max_version,
            flags,
            endpoint_host.into(),
            endpoint_port,
            vendor.into(),
            hardware_version.into(),
            firmware.into(),
            device_id.into(),
        )?;

        Ok(SetupConnection { internal })
    }

    fn get_min_version(&self) -> u16 {
        self.internal.min_version
    }

    fn get_max_version(&self) -> u16 {
        self.internal.max_version
    }

    // fn get_flags(&self) -> &Vec<mining::SetupConnectionFlags> {
    // &self.internal.flags
    // }

    fn get_endpoint_host(&self) -> &str {
        &self.internal.endpoint_host.0
    }

    fn get_vendor(&self) -> &str {
        &self.internal.vendor.0
    }

    fn get_hardware_version(&self) -> &str {
        &self.internal.hardware_version.0
    }

    fn get_firmware(&self) -> &str {
        &self.internal.firmware.0
    }

    fn get_device_id(&self) -> &str {
        &self.internal.device_id.0
    }
}

impl Serializable for SetupConnection {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // TODO: I might as well make the internal just return a Vec<u8>
        let mut buffer = Vec::new();
        self.internal.serialize(&mut buffer)?;
        Ok(writer.write(&buffer)?)
    }
}

/// Implementation of the Framable trait to build a network frame for the
/// SetupConnection message.
impl Framable for SetupConnection {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        // TODO: Call internal.serialize()
        let mut payload = Vec::new();
        let size = *&self.serialize(&mut payload)?;

        // A size_u24 of the message payload.
        let mut payload_length = (size as u16).to_le_bytes().to_vec();
        payload_length.push(0x00);

        let buffer = serialize!(
            &[0x00, 0x00],                           // empty extension type
            &[MessageTypes::SetupConnection.into()], // msg_type
            &payload_length,
            &payload
        );

        Ok(writer.write(&buffer)?)
    }
}

/// OpenStandardMiningChannel is a message sent by the client to the server
/// after a [SetupConnection.Success](struct.SetupConnectionSuccess.html) is
/// sent by the server. This message is used to request opening a standard
/// channel to the upstream server. A standard mining channel indicates `header-only`
/// mining.
pub struct OpenStandardMiningChannel {
    pub request_id: u32,
    pub user_identity: String,
    pub nominal_hash_rate: f32,
    pub max_target: U256,
}

impl OpenStandardMiningChannel {
    /// Constructor for the OpenStandardMiningChannel message.
    fn new(
        request_id: u32,
        user_identity: String,
        nominal_hash_rate: f32,
        max_target: U256,
    ) -> OpenStandardMiningChannel {
        OpenStandardMiningChannel {
            request_id,
            user_identity,
            nominal_hash_rate,
            max_target,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_mining_connection() {
        let message = SetupConnection::new(
            2,
            2,
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
    fn serialize_mining_connection() {
        let message = SetupConnection::new(
            2,
            2,
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
        let message = SetupConnection::new(
            2,
            2,
            vec![],
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
            2,
            2,
            vec![
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
            2,
            2,
            vec![
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
            2,
            2,
            vec![mining::SetupConnectionFlags::RequiresStandardJobs],
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
    fn new_open_standard_mining_channel_0() {
        // TODO: Substitute the nominal hash rate with something more accurate.
        let target = [0u8; 32];

        let message =
            OpenStandardMiningChannel::new(1, "braiinstest.worker1".to_string(), 12.3, target);

        assert_eq!(message.request_id, 1);
        assert_eq!(message.user_identity, "braiinstest.worker1");
        assert_eq!(message.nominal_hash_rate, 12.3);
        assert_eq!(message.max_target.len(), 32);
    }
}
