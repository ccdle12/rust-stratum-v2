use crate::common::BitFlag;

/// Feature flags that can be passed to a JobNegotiationSetupConnection message.
pub enum JobNegotiationSetupConnectionFlags {
    // TODO: Add hyperlinks to all everything between ``
    /// Flag indicating that the `mining_job_token` from `AllocateMiningJobToken.Success`
    /// can be used immediately on a mining connection in `SetCustomMiningJob`
    /// message.
    RequiresAsyncJobMining,
}

impl BitFlag for JobNegotiationSetupConnectionFlags {
    /// Get the byte representation of the flag.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use stratumv2::job_negotiation::messages::JobNegotiationSetupConnectionFlags;
    /// # use stratumv2::common::BitFlag;
    /// let standard_job = JobNegotiationSetupConnectionFlags::RequiresAsyncJobMining.as_byte();
    /// assert_eq!(standard_job, 0b0001);
    /// ```
    fn as_byte(&self) -> u8 {
        match self {
            JobNegotiationSetupConnectionFlags::RequiresAsyncJobMining => 0b0001,
        }
    }
}
