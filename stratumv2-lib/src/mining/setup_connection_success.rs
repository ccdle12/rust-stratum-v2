use crate::impl_bitflags_serde;
use crate::impl_setup_connection_success;

bitflags!(
    /// Feature flags for the SetupConnectionSuccess message from the Server to
    /// the Client for the Mining Protocol.
    pub struct SetupConnectionSuccessFlags: u32 {
        const NONE = 0;
        const REQUIRES_FIXED_VERSION = (1 << 0);
        const REQUIRES_EXTENDED_CHANNELS = (1 << 1);
    }
);

impl_bitflags_serde!(SetupConnectionSuccessFlags, u32);

impl_setup_connection_success!(SetupConnectionSuccessFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::{frame, unframe, Message};
    use crate::parse::{deserialize, serialize};

    #[test]
    fn test_serialize() {
        assert_eq!(
            serialize(&SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION).unwrap(),
            0x01u32.to_le_bytes()
        );
        assert_eq!(
            serialize(&SetupConnectionSuccessFlags::REQUIRES_EXTENDED_CHANNELS).unwrap(),
            0x02u32.to_le_bytes()
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            deserialize::<SetupConnectionSuccessFlags>(&0x01u32.to_le_bytes()).unwrap(),
            SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION,
        );
        assert_eq!(
            deserialize::<SetupConnectionSuccessFlags>(&0x02u32.to_le_bytes()).unwrap(),
            SetupConnectionSuccessFlags::REQUIRES_EXTENDED_CHANNELS,
        );
    }

    #[test]
    fn setup_connection_success() {
        let message =
            SetupConnectionSuccess::new(2, SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION);

        let network_message = frame(&message.unwrap()).unwrap();
        let serialized = serialize(&network_message).unwrap();

        // Check the extension type is empty.
        assert_eq!(serialized[0..=1], [0u8; 2]);

        // Check the msg type is correct.
        assert_eq!(
            serialized[2],
            MessageType::SetupConnectionSuccess.msg_type()
        );

        let der_network_message = deserialize::<Message>(&serialized).unwrap();
        let deserialized = unframe::<SetupConnectionSuccess>(&der_network_message);
        assert!(deserialized.is_ok());
    }
}
