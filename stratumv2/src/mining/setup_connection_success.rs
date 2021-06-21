use crate::{impl_bitflags_serde, impl_setup_connection_success};

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
    use crate::impl_setup_connection_success_tests;
    use crate::parse::{deserialize, serialize};

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
