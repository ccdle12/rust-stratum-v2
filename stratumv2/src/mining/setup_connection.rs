use crate::{impl_bitflags_serde, impl_setup_connection};

bitflags!(
    /// Feature flags that can be passed to a SetupConnection message in the Mining
    /// Protocol. Each flag corresponds to a set bit.
    pub struct SetupConnectionFlags: u32 {
        /// Flag indicating the Client requires Standard Jobs. The Client doesn't
        /// understand group channels and extended jobs.
        const REQUIRES_STANDARD_JOBS = (1 << 0);

        /// Flag indicating that the Client will send the Server a SetCustomMiningJob
        /// message on this connection.
        const REQUIRES_WORK_SELECTION = (1 << 1);

        /// Flag indicating the Client requires version rolling. The Server MUST NOT
        /// send jobs which do not allow version rolling.
        const REQUIRES_VERSION_ROLLING = (1 << 2);
    }
);

impl_bitflags_serde!(SetupConnectionFlags);
impl_setup_connection!(SetupConnectionFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{deserialize, serialize};
    use crate::impl_setup_connection_tests;

    #[test]
    fn flags_serialize() {
        assert_eq!(
            serialize(&SetupConnectionFlags::REQUIRES_STANDARD_JOBS).unwrap(),
            0x01u32.to_le_bytes()
        );
        assert_eq!(
            serialize(&SetupConnectionFlags::REQUIRES_WORK_SELECTION).unwrap(),
            0x02u32.to_le_bytes()
        );
        assert_eq!(
            serialize(&SetupConnectionFlags::REQUIRES_VERSION_ROLLING).unwrap(),
            0x04u32.to_le_bytes()
        );
    }

    #[test]
    fn flags_deserialize() {
        assert_eq!(
            deserialize::<SetupConnectionFlags>(&0x01u32.to_le_bytes()).unwrap(),
            SetupConnectionFlags::REQUIRES_STANDARD_JOBS,
        );
        assert_eq!(
            deserialize::<SetupConnectionFlags>(&0x02u32.to_le_bytes()).unwrap(),
            SetupConnectionFlags::REQUIRES_WORK_SELECTION,
        );
        assert_eq!(
            deserialize::<SetupConnectionFlags>(&0x04u32.to_le_bytes()).unwrap(),
            SetupConnectionFlags::REQUIRES_VERSION_ROLLING,
        );

        assert!(matches!(
            deserialize::<SetupConnectionFlags>(&0xffu32.to_le_bytes()),
            Err(Error::UnknownFlags { .. })
        ));
    }

    impl_setup_connection_tests!(SetupConnectionFlags);
}
