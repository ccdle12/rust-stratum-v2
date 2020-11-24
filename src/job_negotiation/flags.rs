use crate::common::BitFlag;

/// Feature flags that can be passed to a SetupConnection message for the
/// job negotiation protocol. Each flag corresponds to a set bit.
pub enum SetupConnectionFlags {
    // TODO: Add hyperlinks to all everything between ``
    /// Flag indicating that the `mining_job_token` from `AllocateMiningJobToken.Success`
    /// can be used immediately on a mining connection in `SetCustomMiningJob`
    /// message.
    RequiresAsyncJobMining,
}

impl BitFlag for SetupConnectionFlags {
    /// Get the byte representation of the flag.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use stratumv2::job_negotiation::SetupConnectionFlags;
    /// # use stratumv2::common::BitFlag;
    ///
    /// let standard_job = SetupConnectionFlags::RequiresAsyncJobMining.as_byte();
    /// assert_eq!(standard_job, 0b0001);
    /// ```
    fn as_byte(&self) -> u8 {
        match self {
            SetupConnectionFlags::RequiresAsyncJobMining => 0b0001,
        }
    }
}
