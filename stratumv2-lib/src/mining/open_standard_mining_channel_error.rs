use crate::impl_message;
use crate::impl_open_mining_channel_error;
use crate::types::MessageType;

impl_open_mining_channel_error!(
    OpenStandardMiningChannelError,
    MessageType::OpenStandardMiningChannelError
);
