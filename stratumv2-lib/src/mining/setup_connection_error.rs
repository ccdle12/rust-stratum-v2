use crate::common::SetupConnectionErrorCode;
use crate::impl_setup_connection_error;
use crate::mining::SetupConnectionFlags;

impl_setup_connection_error!(SetupConnectionFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{frame, unframe, Message};
    use crate::parse::{deserialize, serialize};

    #[test]
    fn frame_connection_error() {
        let message = SetupConnectionError::new(
            SetupConnectionFlags::REQUIRES_STANDARD_JOBS,
            SetupConnectionErrorCode::UnsupportedFeatureFlags,
        )
        .unwrap();

        let network_message = frame(&message).unwrap();
        let serialized = serialize(&network_message).unwrap();

        // Check the extension type is empty.
        assert_eq!(serialized[0..=1], [0u8; 2]);

        // Check the msg type is correct.
        assert_eq!(serialized[2], MessageType::SetupConnectionError.msg_type());

        let der_network_message = deserialize::<Message>(&serialized).unwrap();
        let deserialized = unframe::<SetupConnectionError>(&der_network_message);
        assert!(deserialized.is_ok());
    }
}
