use crate::common::BitFlag;

/// Feature flags that can be passed to a MiningSetupConnection message. Each
/// flag corresponds to a set bit.
pub enum MiningSetupConnectionFlags {
    /// Flag indicating the downstream node requires standard jobs. The node
    /// doesn't undestand group channels and extended jobs.
    RequiresStandardJobs,

    /// Flag indicating that the client will send the server SetCustomMiningJob
    /// message on this connection.
    RequiresWorkSelection,

    /// Flag indicating the client requires version rolling. The server MUST NOT
    /// send jobs which do not allow version rolling.
    RequiresVersionRolling,
}

impl BitFlag for MiningSetupConnectionFlags {
    /// Get the byte representation of the flag.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use stratumv2::mining::messages::MiningSetupConnectionFlags;
    /// # use stratumv2::common::BitFlag;
    ///
    /// let standard_job = MiningSetupConnectionFlags::RequiresStandardJobs.as_byte();
    /// assert_eq!(standard_job, 0b0001);
    /// ```
    fn as_byte(&self) -> u8 {
        match self {
            MiningSetupConnectionFlags::RequiresStandardJobs => 0b0001,
            MiningSetupConnectionFlags::RequiresWorkSelection => 0b0010,
            MiningSetupConnectionFlags::RequiresVersionRolling => 0b0100,
        }
    }
}
