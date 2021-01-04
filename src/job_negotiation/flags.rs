use crate::common::messages::Protocol;
use crate::common::{BitFlag, ToProtocol};

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
    /// use stratumv2::job_negotiation::SetupConnectionFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let standard_job = SetupConnectionFlags::RequiresAsyncJobMining.as_bytes();
    /// assert_eq!(standard_job, 0x01);
    /// ```
    fn as_bytes(&self) -> u32 {
        match self {
            SetupConnectionFlags::RequiresAsyncJobMining => 1,
        }
    }
}

impl ToProtocol for SetupConnectionFlags {
    fn as_protocol(&self) -> Protocol {
        Protocol::JobNegotiation
    }
}
