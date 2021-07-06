use rand::Rng;

/// ChanID is a type assigned to identify channels on a connection.
/// The type de/serialzing for messaging is already handled using the native
/// type u32.
pub type ChanID = u32;

/// Generate a new random channel_id.
pub fn new_channel_id() -> ChanID {
    rand::thread_rng().gen_range(0, ChanID::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_id_generate() {
        assert!(new_channel_id() <= u32::MAX)
    }
}
