use crate::common::SetupConnection;

/// An enum that contains variants for each message that can be sent out on the
/// wire. This enum is primarily used to cache outgoing messages on a buffer
/// on each device.
pub enum MessageSendEvent {
    SetupConnectionEvent { msg: SetupConnection },
}
