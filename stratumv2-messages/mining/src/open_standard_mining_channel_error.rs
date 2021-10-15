use crate::impl_open_mining_channel_error;

// OpenStandardMiningChannelError is an implementation of the OpeningMiningChannelError
// for Standard Channels. The MessageType for this message will be 0x12.
impl_open_mining_channel_error!(OpenStandardMiningChannelError);

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_open_mining_channel_error_tests;

    impl_open_mining_channel_error_tests!(OpenStandardMiningChannelError);
}
