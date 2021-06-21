use crate::impl_message;
use crate::impl_open_mining_channel_error;
use crate::mining::OpenMiningChannelErrorCode;
use crate::types::MessageType;

impl_open_mining_channel_error!(OpenExtendedMiningChannelError);

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_open_mining_channel_error_tests;

    impl_open_mining_channel_error_tests!(OpenExtendedMiningChannelError);
}
