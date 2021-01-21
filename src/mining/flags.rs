use crate::common::{BitFlag, Protocol, ToProtocol};

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

impl BitFlag for SetupConnectionFlags {
    /// Gets the set bit representation of a SetupConnectionFlag as a u32.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::mining::SetupConnectionFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let standard_job = SetupConnectionFlags::RequiresStandardJobs.as_bit_flag();
    /// assert_eq!(standard_job, 0x01);
    /// ```
    fn as_bit_flag(&self) -> u32 {
        match self {
            SetupConnectionFlags::RequiresStandardJobs => 1,
            SetupConnectionFlags::RequiresWorkSelection => (1 << 1),
            SetupConnectionFlags::RequiresVersionRolling => (1 << 2),
        }
    }

    /// Gets a vector of flag enum representations from a u32.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::mining::SetupConnectionFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let flags = SetupConnectionFlags::deserialize_flags(7);
    /// assert_eq!(flags[0], SetupConnectionFlags::RequiresStandardJobs);
    /// assert_eq!(flags[1], SetupConnectionFlags::RequiresWorkSelection);
    /// assert_eq!(flags[2], SetupConnectionFlags::RequiresVersionRolling);
    /// ```
    fn deserialize_flags(flags: u32) -> Vec<SetupConnectionFlags> {
        let mut der_flags = Vec::new();

        if flags & SetupConnectionFlags::RequiresStandardJobs.as_bit_flag() != 0 {
            der_flags.push(SetupConnectionFlags::RequiresStandardJobs)
        }

        if flags & SetupConnectionFlags::RequiresWorkSelection.as_bit_flag() != 0 {
            der_flags.push(SetupConnectionFlags::RequiresWorkSelection)
        }

        if flags & SetupConnectionFlags::RequiresVersionRolling.as_bit_flag() != 0 {
            der_flags.push(SetupConnectionFlags::RequiresVersionRolling)
        }

        der_flags
    }
}

/// Implement ToProtocol to be able to match the flags to a specific Stratum V2
/// Protocol.
impl ToProtocol for SetupConnectionFlags {
    fn as_protocol(&self) -> Protocol {
        Protocol::Mining
    }
}

/// Feature flags for the SetupConnectionSuccess message from the Server to the
/// Cliennt, specifically for the Mining Protocol.
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

impl BitFlag for SetupConnectionSuccessFlags {
    fn as_bit_flag(&self) -> u32 {
        match self {
            SetupConnectionSuccessFlags::RequiresFixedVersion => 1,
            SetupConnectionSuccessFlags::RequiresExtendedChannels => (1 << 1),
        }
    }

    /// Gets a vector of flag enum representations from a u32.
    ///
    /// Example:
    ///
    /// ```rust
    /// use stratumv2::mining::SetupConnectionSuccessFlags;
    /// use stratumv2::common::BitFlag;
    ///
    /// let flags = SetupConnectionSuccessFlags::deserialize_flags(3);
    /// assert_eq!(flags[0], SetupConnectionSuccessFlags::RequiresFixedVersion);
    /// assert_eq!(flags[1], SetupConnectionSuccessFlags::RequiresExtendedChannels);
    /// ```
    fn deserialize_flags(flags: u32) -> Vec<SetupConnectionSuccessFlags> {
        let mut der_flags = Vec::new();

        if flags & SetupConnectionSuccessFlags::RequiresFixedVersion.as_bit_flag() != 0 {
            der_flags.push(SetupConnectionSuccessFlags::RequiresFixedVersion)
        }

        if flags & SetupConnectionSuccessFlags::RequiresExtendedChannels.as_bit_flag() != 0 {
            der_flags.push(SetupConnectionSuccessFlags::RequiresExtendedChannels)
        }

        der_flags
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u32_to_vec_flags() {
        let set_flags = 7;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);

        assert_eq!(flags.len(), 3);
        assert_eq!(flags[0], SetupConnectionFlags::RequiresStandardJobs);
        assert_eq!(flags[1], SetupConnectionFlags::RequiresWorkSelection);
        assert_eq!(flags[2], SetupConnectionFlags::RequiresVersionRolling);

        let set_flags = 3;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);
        assert_eq!(flags.len(), 2);
        assert_eq!(flags[0], SetupConnectionFlags::RequiresStandardJobs);
        assert_eq!(flags[1], SetupConnectionFlags::RequiresWorkSelection);

        let set_flags = 2;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0], SetupConnectionFlags::RequiresWorkSelection);

        let set_flags = 8;
        let flags = SetupConnectionFlags::deserialize_flags(set_flags);
        assert_eq!(flags.len(), 0);
    }
}
