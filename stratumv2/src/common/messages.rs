use crate::error::{Error, Result};
use crate::types::MessageTypes;
use crate::Deserializable;
use std::convert::TryInto;
use std::fmt;

/// Contains the error codes for the [SetupConnectionError](struct.SetupConnectionError.html) message.
/// Each error code has a default STR0_255 message.
#[derive(Debug, PartialEq, Clone, Copy)]
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

    // TODO: Review this, I don't like it
    UnknownFlag,
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

            // TODO: Review this, I don't like it
            SetupConnectionErrorCodes::UnknownFlag => write!(f, "unknown flag"),
        }
    }
}

impl From<&str> for SetupConnectionErrorCodes {
    fn from(error_code: &str) -> Self {
        match error_code {
            "unsupported-feature-flags" => SetupConnectionErrorCodes::UnsupportedFeatureFlags,
            "unsupported-protocol" => SetupConnectionErrorCodes::UnsupportedProtocol,
            "protocol-version-mismatch" => SetupConnectionErrorCodes::ProtocolVersionMismatch,

            // TODO: Review this, I don't like it
            _ => SetupConnectionErrorCodes::UnknownFlag,
        }
    }
}

// TODO: Docstring
// Example:
pub struct NetworkFrame {
    pub extension_type: u16,
    pub msg_type: MessageTypes,
    // TODO: decode the le U24 to u32.
    pub msg_length: u32,
    pub payload: Vec<u8>,
}

impl Deserializable for NetworkFrame {
    fn deserialize(bytes: &[u8]) -> Result<NetworkFrame> {
        // Get the extension type.
        let start = 0;
        let offset = start + 2;
        let extension_type_bytes = bytes.get(start..offset);
        if extension_type_bytes.is_none() {
            return Err(Error::DeserializationError(
                "received empty bytes in network frame".into(),
            ));
        }

        let extension_type = u16::from_le_bytes(extension_type_bytes.unwrap().try_into().unwrap());

        // Get the message type.
        let offset = 2;
        let msg_type_bytes = bytes.get(offset);

        if msg_type_bytes.is_none() {
            return Err(Error::DeserializationError(
                "missing msg_type in network frame".into(),
            ));
        }
        let msg_type = MessageTypes::from(*msg_type_bytes.unwrap());

        // Get the length of the payload.
        let start = offset;
        let offset = start + 3;

        // TODO: Create a function that changes U24 to u16.
        let msg_length_bytes = bytes.get(start..offset);
        if msg_length_bytes.is_none() {
            return Err(Error::DeserializationError(
                "missing message length in network frame".into(),
            ));
        }

        // TODO: Need to review this
        let msg_length = (msg_length_bytes.unwrap()[2] as u32)
            | (msg_length_bytes.unwrap()[1] as u32)
            | (msg_length_bytes.unwrap()[0] as u32);

        // Get the variable length payload.
        let start = offset + 1;
        let offset = start + msg_length as usize;

        let payload = bytes.get(start..offset);
        if payload.is_none() {
            return Err(Error::DeserializationError(
                "missing payload in network frame".into(),
            ));
        }

        Ok(NetworkFrame {
            extension_type,
            msg_type,
            msg_length,
            payload: payload.unwrap().to_vec(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mining;

    #[test]
    fn deserialize_network_frame() {
        let input = [
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
            0x68, // firmware
            0x09, // length_device_id
            0x73, 0x6f, 0x6d, 0x65, 0x2d, 0x75, 0x75, 0x69, 0x64, // device_id
        ];

        let network_frame = NetworkFrame::deserialize(&input).unwrap();
        assert_eq!(network_frame.extension_type, 0);
        assert_eq!(
            MessageTypes::from(network_frame.msg_type),
            MessageTypes::SetupConnection
        );

        assert!(mining::SetupConnection::deserialize(&network_frame.payload).is_ok());
    }
}
