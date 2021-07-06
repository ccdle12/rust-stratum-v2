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
pub struct Channel {
    /// The unique ID assigned to a channel for both Upstream and Downstream
    /// devices.
    pub chan_id: ChanID,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_id_generate() {
        assert!(new_channel_id() <= u32::MAX)
    }
}
