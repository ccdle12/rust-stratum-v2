use crate::error::Result;
use std::time::SystemTime;

/// Convert SystemTime to a Unix timestamp as a u32.
pub fn system_unix_time_to_u32(time: &SystemTime) -> Result<u32> {
    Ok(time
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as u32)?)
}

/// A helper function that returns the current unix time in seconds as a u32.
pub fn unix_u32_now() -> Result<u32> {
    system_unix_time_to_u32(&SystemTime::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::error::Error;
    use std::time::Duration;

    #[test]
    fn system_unix_time_epoch() {
        let five_after = SystemTime::UNIX_EPOCH
            .checked_add(Duration::new(5, 0))
            .unwrap();
        assert_eq!(system_unix_time_to_u32(&five_after).unwrap(), 5u32);
    }

    #[test]
    fn system_unix_time_resolution() {
        let five_after = SystemTime::UNIX_EPOCH
            .checked_add(Duration::new(5, 3))
            .unwrap();
        assert_eq!(system_unix_time_to_u32(&five_after).unwrap(), 5u32);
    }

    #[test]
    fn system_unix_time_error() {
        let five_till = SystemTime::UNIX_EPOCH
            .checked_sub(Duration::new(5, 3))
            .unwrap();
        assert!(matches!(
            system_unix_time_to_u32(&five_till),
            Err(Error::SystemTimeError { .. })
        ));
    }

    #[test]
    fn unix_u32_now_sequence() {
        let first = unix_u32_now().unwrap();
        let second = unix_u32_now().unwrap();
        assert!(first <= second);
    }
}
