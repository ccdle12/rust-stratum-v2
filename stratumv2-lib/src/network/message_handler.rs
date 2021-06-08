// TODO:
// 1. Is it flexible enough to handle different types of requirements for each
// handler function?
//   - I was thinking we could pass around a Context { ... } that has Options
//   on the different types, separate from Message which should contain configs
use crate::{
    error::{Error, Result},
    frame::Message,
    types::MessageType,
};
use std::collections::HashMap;

/// An type alias for handler functions according to a received NetworkMessage.
type HandlerFunction = fn(message: &Message) -> Result<()>;

/// MessageHandler is struct that contains a mapping of MessageType to
/// a HandlerFunction. This allows the caller to assign their own function
/// implementations according to certain MessageTypes.
///
/// # Examples
///
/// ```rust
/// use stratumv2_lib::{
///     error::Result,
///     network::MessageHandler,
///     frame::Message,
///     types::MessageType,
///     msg_handler,
/// };
///
/// fn setup_conn_success_handler(message: &Message) -> Result<()> {
///   println!("received a setup connection message");
///   Ok(())
/// }
///
/// fn open_extended_mining_channel_handler(message: &Message) -> Result<()> {
///   println!("received an open extended channel message");
///   Ok(())
/// }
///
/// let message_handler = msg_handler!(
///   MessageType::OpenExtendedMiningChannel => open_extended_mining_channel_handler,
///   MessageType::SetupConnectionSuccess => setup_conn_success_handler
/// );
/// ```
pub struct MessageHandler {
    pub handlers: HashMap<MessageType, HandlerFunction>,
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {
            handlers: HashMap::new(),
        }
    }

    /// Receive a Message from a networked source and call the handler for the
    /// associated MessageType, if it exists.
    pub fn receive_message(&self, message: &Message) -> Result<()> {
        match self.handlers.get(&message.message_type) {
            Some(function) => function(message),
            // TODO: Return another error.
            None => Err(Error::Unimplemented()),
        }
    }
}

#[macro_export]
macro_rules! msg_handler {
    ($($message_type:path => $handler_func:expr),*) => {{
        let mut message_handler = MessageHandler::new();
        $(message_handler.handlers.insert($message_type, $handler_func));*;

        message_handler
    }}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        frame::frame,
        mining::{SetupConnectionSuccess, SetupConnectionSuccessFlags},
    };

    fn default_message() -> Message {
        let conn_success =
            SetupConnectionSuccess::new(2, SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION)
                .unwrap();

        frame(&conn_success).unwrap()
    }

    #[test]
    fn init_message_handler() {
        let handler = msg_handler!(
            MessageType::SetupConnectionSuccess => |_msg: &Message| Ok(()),
            MessageType::SetupConnectionError => |_msg: &Message| Ok(())
        );

        let message = default_message();
        assert!(handler.receive_message(&message).is_ok());
    }

    #[test]
    fn unregistered_handler() {
        let handler = MessageHandler::new();

        let message = default_message();
        assert!(handler.receive_message(&message).is_err());
    }
}
