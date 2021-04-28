use crate::impl_message;
use crate::impl_open_mining_channel_error;
use crate::mining::OpenMiningChannelErrorCode;
use crate::types::MessageType;

impl_open_mining_channel_error!(
    OpenExtendedMiningChannelError,
    MessageType::OpenExtendedMiningChannelError
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::{frame, unframe, Message};
    use crate::parse::{deserialize, serialize};

    #[test]
    fn frame_open_extended_mining_channel_error() {
        let message =
            OpenExtendedMiningChannelError::new(1, OpenMiningChannelErrorCode::UnknownUser)
                .unwrap();

        let network_message = frame(&message).unwrap();
        assert_eq!(
            network_message.message_type,
            MessageType::OpenExtendedMiningChannelError
        );

        let serialized = serialize(&network_message).unwrap();

        // Check the correct message type was used.
        assert_eq!(
            serialized[2],
            MessageType::OpenExtendedMiningChannelError.msg_type()
        );

        // Check the extension type is empty for this message.
        assert_eq!(serialized[0..=1], [0u8; 2]);

        // Check the serialized frame can be deserialized for this message.
        let der_message = deserialize::<Message>(&serialized).unwrap();
        let der_mining_channel_error = unframe::<OpenExtendedMiningChannelError>(&der_message);

        assert!(der_mining_channel_error.is_ok());
        assert_eq!(der_mining_channel_error.unwrap(), message);
    }
}
