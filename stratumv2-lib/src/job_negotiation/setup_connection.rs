use crate::{impl_bitflags_serde, impl_setup_connection};

bitflags!(
  /// Feature flags that can be passed to a SetupConnection message for the
  /// job negotiation protocol. Each flag corresponds to a set bit.
  pub struct SetupConnectionFlags: u32 {
    // TODO: Add hyperlinks to all everything between ``
    /// Flag indicating that the `mining_job_token` from `AllocateMiningJobToken.Success`
    /// can be used immediately on a mining connection in `SetCustomMiningJob`
    /// message.
    const REQUIRES_ASYNC_JOB_MINING = (1 << 0);
  }
);

impl_bitflags_serde!(SetupConnectionFlags, u32);

impl_setup_connection!(SetupConnectionFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_setup_connection_tests;
    use crate::parse::{deserialize, serialize};

    #[test]
    fn flags_serialize() {
        assert_eq!(
            serialize(&SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING).unwrap(),
            0x01u32.to_le_bytes()
        );
    }

    #[test]
    fn flags_deserialize() {
        assert_eq!(
            deserialize::<SetupConnectionFlags>(&0x01u32.to_le_bytes()).unwrap(),
            SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING,
        );
    }

    impl_setup_connection_tests!(SetupConnectionFlags);
}
