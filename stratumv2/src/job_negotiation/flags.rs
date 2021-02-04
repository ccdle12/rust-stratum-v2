use crate::BitFlag;

/// Feature flags that can be passed to a SetupConnection message for the
/// job negotiation protocol. Each flag corresponds to a set bit.
#[derive(Debug, PartialEq, Clone)]
pub enum SetupConnectionFlags {
    // TODO: Add hyperlinks to all everything between ``
    /// Flag indicating that the `mining_job_token` from `AllocateMiningJobToken.Success`
    /// can be used immediately on a mining connection in `SetCustomMiningJob`
    /// message.
    RequiresAsyncJobMining,
}

impl_message_flag!(SetupConnectionFlags, SetupConnectionFlags::RequiresAsyncJobMining => 0);
