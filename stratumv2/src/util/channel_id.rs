use rand::Rng;

/// Generate a new random channel_id.
pub fn new_channel_id() -> u32 {
    rand::thread_rng().gen_range(0, u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_id_generate() {
        assert!(new_channel_id() <= u32::MAX)
    }
}
