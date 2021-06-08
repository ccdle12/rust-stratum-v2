use crate::error::{Error, Result};

/// MessageType contains all the variations for the byte representation of
/// messages used in message frames.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum MessageType {
    // Common messages
    SetupConnection,
    SetupConnectionSuccess,
    SetupConnectionError,
    ChannelEndpointChanged,
    // Mining protocol messages
    OpenStandardMiningChannel,
    OpenStandardMiningChannelSuccess,
    OpenStandardMiningChannelError,
    OpenExtendedMiningChannel,
    OpenExtendedMiningChannelSuccess,
    OpenExtendedMiningChannelError,
    UpdateChannel,
    UpdateChannelError,
    CloseChannel,
    SetExtranoncePrefix,
    SubmitSharesStandard,
    SubmitSharesExtended,
    SubmitSharesSuccess,
    SubmitSharesError,
    NewMiningJob,
    NewExtendedMiningJob,
    SetNewPrevHash,
    SetTarget,
    SetCustomMiningJob,
    SetCustomMiningJobSuccess,
    SetCustomMiningJobError,
    Reconnect,
    SetGroupChannel,
    // TODO(chpatton013): Job negotiation protocol messages
    // TODO(chpatton013): Template distribution protocol messages
    // Testing messages
    TestMessage1,
    TestMessage2,
}

macro_rules! impl_enum_message_types {
    ($type:ident, $($variant:path => ($ext_type:expr, $msg_type:expr, $channel_bit:expr)),*) => {
        impl $type {
            pub fn new(ext_type: u16, msg_type: u8) -> Result<$type> {
                match (ext_type, msg_type) {
                    $(($ext_type, $msg_type) => Ok($variant)),*,
                    _ => Err(Error::UnknownMessageType()),
                }
            }

            pub fn ext_type(&self) -> u16 {
                match self {
                    $($variant => $ext_type),*
                }
            }

            pub fn msg_type(&self) -> u8 {
                match self {
                    $($variant => $msg_type),*
                }
            }

            pub fn channel_bit(&self) -> bool {
                match self {
                    $($variant => $channel_bit),*
                }
            }
        }
    };
}

impl_enum_message_types!(
    MessageType,
    MessageType::SetupConnection => (0x0000, 0x00, false),
    MessageType::SetupConnectionSuccess => (0x0000, 0x01, false),
    MessageType::SetupConnectionError => (0x0000, 0x02, false),
    MessageType::ChannelEndpointChanged => (0x0000, 0x03, true),
    MessageType::OpenStandardMiningChannel => (0x0000, 0x10, false),
    MessageType::OpenStandardMiningChannelSuccess => (0x0000, 0x11, false),
    MessageType::OpenStandardMiningChannelError => (0x0000, 0x12, false),
    MessageType::OpenExtendedMiningChannel => (0x0000, 0x13, false),
    MessageType::OpenExtendedMiningChannelSuccess => (0x0000, 0x14, false),
    MessageType::OpenExtendedMiningChannelError => (0x0000, 0x15, false),
    MessageType::UpdateChannel => (0x0000, 0x16, true),
    MessageType::UpdateChannelError => (0x0000, 0x17, true),
    MessageType::CloseChannel => (0x0000, 0x18, true),
    MessageType::SetExtranoncePrefix => (0x0000, 0x19, true),
    MessageType::SubmitSharesStandard => (0x0000, 0x1a, true),
    MessageType::SubmitSharesExtended => (0x0000, 0x1b, true),
    MessageType::SubmitSharesSuccess => (0x0000, 0x1c, true),
    MessageType::SubmitSharesError => (0x0000, 0x1d, true),
    MessageType::NewMiningJob => (0x0000, 0x1e, true),
    MessageType::NewExtendedMiningJob => (0x0000, 0x1f, true),
    MessageType::SetNewPrevHash => (0x0000, 0x20, true),
    MessageType::SetTarget => (0x0000, 0x21, true),
    MessageType::SetCustomMiningJob => (0x0000, 0x22, false),
    MessageType::SetCustomMiningJobSuccess => (0x0000, 0x23, false),
    MessageType::SetCustomMiningJobError => (0x0000, 0x24, false),
    MessageType::Reconnect => (0x0000, 0x25, false),
    MessageType::SetGroupChannel => (0x0000, 0x26, false),
    MessageType::TestMessage1 => (0x0000, 0xfe, false),
    MessageType::TestMessage2 => (0x0000, 0xff, true)
);
