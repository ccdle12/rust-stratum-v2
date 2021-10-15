use stratumv2_messages_sdk::impl_setup_connection_success;
use stratumv2_serde::impl_bitflags_serde;

bitflags!(
    /// Feature flags for the SetupConnectionSuccess message from the Server to
    /// the Client for the Mining Protocol.
    pub struct SetupConnectionSuccessFlags: u32 {
        const NONE = 0;
        const REQUIRES_FIXED_VERSION = (1 << 0);
        const REQUIRES_EXTENDED_CHANNELS = (1 << 1);
    }
);

impl_bitflags_serde!(SetupConnectionSuccessFlags);

// SetupConnectionSuccess is an implementation of the SetupConnectionSuccess
// message specific to the mining subprotocol and will contain the mining
// SetupConnectionSuccessFlags.
impl_setup_connection_success!(SetupConnectionSuccessFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use stratumv2_messages_sdk::impl_setup_connection_success_tests;
    use stratumv2_serde::{deserialize, serialize};

    #[test]
    fn flags_serialize() {
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
    fn flags_deserialize() {
        assert_eq!(
            deserialize::<SetupConnectionSuccessFlags>(&0x01u32.to_le_bytes()).unwrap(),
            SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION,
        );
        assert_eq!(
            deserialize::<SetupConnectionSuccessFlags>(&0x02u32.to_le_bytes()).unwrap(),
            SetupConnectionSuccessFlags::REQUIRES_EXTENDED_CHANNELS,
        );
    }

    impl_setup_connection_success_tests!(SetupConnectionSuccessFlags);
}
