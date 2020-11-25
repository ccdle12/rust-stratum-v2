use crate::common::BitFlag;

/// Feature flags that can be passed to a SetupConnection message in the Mining
/// Protocol. Each flag corresponds to a set bit.
pub enum SetupConnectionFlags {
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

impl BitFlag for SetupConnectionFlags {
    /// Get the byte representation of the flag.
    ///
    /// Example:
    ///
    /// ```rust
    /// # use stratumv2::mining::SetupConnectionFlags;
    /// # use stratumv2::common::BitFlag;
    ///
    /// let standard_job = SetupConnectionFlags::RequiresStandardJobs.as_byte();
    /// assert_eq!(standard_job, 0b0001);
    /// ```
    fn as_byte(&self) -> u8 {
        match self {
            SetupConnectionFlags::RequiresStandardJobs => 0b0001,
            SetupConnectionFlags::RequiresWorkSelection => 0b0010,
            SetupConnectionFlags::RequiresVersionRolling => 0b0100,
        }
    }
}

/// Feature flags for the SetupConnectionSuccess message from the server to
/// the clien for the mining protocol.
pub enum SetupConnectionSuccessFlags {
    // TODO: Link everthing between ``
    /// Flag indicating the upstream node does not accept any changes to the
    /// version field. If `RequiresVersionRolling` was sent in the `SetupConnection`
    /// message, then this bit MUST NOT be set.
    RequiresFixedVersion,

    /// Flag indicating that the upstream node will not accept opening a
    /// standard channel.
    RequiresExtendedChannels,
}

impl BitFlag for SetupConnectionSuccessFlags {
    fn as_byte(&self) -> u8 {
        match self {
            SetupConnectionSuccessFlags::RequiresFixedVersion => 0b0001,
            SetupConnectionSuccessFlags::RequiresExtendedChannels => 0b0010,
        }
    }
}
