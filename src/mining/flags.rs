use crate::common::{BitFlag, Protocol, ToProtocol};

/// Feature flags that can be passed to a SetupConnection message in the mining
/// sub protocol. Each flag corresponds to a set bit.
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
    /// Gets the set bit representation of a u32.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::mining::SetupConnectionFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let standard_job = SetupConnectionFlags::RequiresStandardJobs.as_bytes();
    /// assert_eq!(standard_job, 0x01);
    /// ```
    fn as_bytes(&self) -> u32 {
        match self {
            SetupConnectionFlags::RequiresStandardJobs => 1,
            SetupConnectionFlags::RequiresWorkSelection => (1 << 1),
            SetupConnectionFlags::RequiresVersionRolling => (1 << 2),
        }
    }
}

/// Implement ToProtocol to be able to match the flags to a specific Stratum V2
/// Protocol.
impl ToProtocol for SetupConnectionFlags {
    fn as_protocol(&self) -> Protocol {
        Protocol::Mining
    }
}

/// Feature flags for the SetupConnectionSuccess message from the server to
/// the client for the mining protocol.
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
    fn as_bytes(&self) -> u32 {
        match self {
            SetupConnectionSuccessFlags::RequiresFixedVersion => 1,
            SetupConnectionSuccessFlags::RequiresExtendedChannels => (1 << 1),
        }
    }
}

impl ToProtocol for SetupConnectionSuccessFlags {
    fn as_protocol(&self) -> Protocol {
        Protocol::Mining
    }
}
