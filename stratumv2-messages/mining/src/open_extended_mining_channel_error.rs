use crate::impl_open_mining_channel_error;

// OpenExtendedMiningChannelError is an implementation of the OpeningMiningChannelError
// message for Extended Channels. The MessageType for this message will be 0x15.
impl_open_mining_channel_error!(OpenExtendedMiningChannelError);

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_open_mining_channel_error_tests;

    impl_open_mining_channel_error_tests!(OpenExtendedMiningChannelError);
}
