use crate::common::{BitFlag, Protocol, ToProtocol};

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

impl BitFlag for SetupConnectionFlags {
    /// Gets the set bit representation of a SetupConnectionFlag as a u32.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::job_negotiation::SetupConnectionFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let standard_job = SetupConnectionFlags::RequiresAsyncJobMining.as_bit_flag();
    /// assert_eq!(standard_job, 0x01);
    /// ```
    fn as_bit_flag(&self) -> u32 {
        match self {
            SetupConnectionFlags::RequiresAsyncJobMining => 1,
        }
    }

    /// Gets a vector of flag enum representations from a u32.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::job_negotiation::SetupConnectionFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let flags = SetupConnectionFlags::deserialize_flags(1);
    /// assert_eq!(flags[0], SetupConnectionFlags::RequiresAsyncJobMining);
    /// ```
    fn deserialize_flags(flags: u32) -> Vec<SetupConnectionFlags> {
        let mut der_flags = Vec::new();

        if flags & SetupConnectionFlags::RequiresAsyncJobMining.as_bit_flag() != 0 {
            der_flags.push(SetupConnectionFlags::RequiresAsyncJobMining)
        }

        der_flags
    }
}

impl ToProtocol for SetupConnectionFlags {
    fn as_protocol(&self) -> Protocol {
        Protocol::JobNegotiation
    }
}
