use crate::BitFlag;

/// Feature flags that can be passed to a SetupConnection message in the mining
/// sub protocol. Each flag corresponds to a set bit.
#[derive(Debug, PartialEq, Clone)]
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

impl_message_flag!(
    SetupConnectionFlags,
    SetupConnectionFlags::RequiresStandardJobs => 0,
    SetupConnectionFlags::RequiresWorkSelection => 1,
    SetupConnectionFlags::RequiresVersionRolling => 2
);

/// Feature flags for the SetupConnectionSuccess message from the server to
/// the client for the mining protocol.
#[derive(Debug, PartialEq, Clone)]
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

impl_message_flag!(
    SetupConnectionSuccessFlags,
    SetupConnectionSuccessFlags::RequiresFixedVersion => 0,
    SetupConnectionSuccessFlags::RequiresExtendedChannels => 1
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u32_deserialize_flags() {
        let set_flags = 7;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);

        assert_eq!(flags.len(), 3);
        assert_eq!(
            flags,
            &[
                SetupConnectionFlags::RequiresStandardJobs,
                SetupConnectionFlags::RequiresWorkSelection,
                SetupConnectionFlags::RequiresVersionRolling
            ]
        );

        let set_flags = 3;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);
        assert_eq!(
            flags,
            &[
                SetupConnectionFlags::RequiresStandardJobs,
                SetupConnectionFlags::RequiresWorkSelection,
            ]
        );

        let set_flags = 2;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0], SetupConnectionFlags::RequiresWorkSelection);

        let set_flags = 8;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);
        assert_eq!(flags.len(), 0);
    }
}
