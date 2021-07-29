use crate::mining::{OpenExtendedMiningChannel, OpenStandardMiningChannel};
use std::{collections::HashMap, sync::Mutex};

use rand::Rng;

/// ChanID is a type assigned to identify channels on a connection.
/// The type de/serialzing for messaging is already handled using the native
/// type u32.
pub type ChanID = u32;

/// Generate a new random channel_id.
pub fn new_channel_id() -> ChanID {
    rand::thread_rng().gen_range(0, ChanID::MAX)
}

/// Represents a Channel opened on a connection. The Channel holds stateful
/// information about the Channel for both Upstream and Downstream devices.
#[derive(Debug, Clone)]
pub enum Channel {
    StandardMiningChannel {
        id: ChanID,
        channel: OpenStandardMiningChannel,
    },
    ExtendedMiningChannel {
        id: ChanID,
        channel: OpenExtendedMiningChannel,
    },
}

// TODO: Replace the u32 with a ConnID type.
/// Holds a collection of [Channels](./enum.Channel.html) according to the ChanID and linked to a
/// ConnID, representing the networked connection.
pub struct ChannelManager {
    /// Contains multiple channels that belong to a certain connection according
    /// to the Connection ID.
    pub channels: Mutex<HashMap<u32, HashMap<ChanID, Channel>>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            channels: Mutex::new(HashMap::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_id_generate() {
        assert!(new_channel_id() <= u32::MAX)
    }
}
