use std::{mem, sync::Mutex};
use stratumv2_codec::Message;
use stratumv2_common_messages::SetupConnection;
use stratumv2_noise::Session;

/// Peer holds state information about a Connection. It distinctly does NOT
/// hold any network structs such as TCPStreams, only state and logic required
/// to execute the business logic of the device.
pub struct Peer<S: Session> {
    /// An encryptor used to de/encrypt messages on this connection.
    pub encryptor: S,

    /// The required SetupConnection message on this connection. If this message
    /// doesn't exist then we'll assume as a Server, we are waiting to receive
    /// one and won't process any further stratumv2 messages. If as a Client,
    /// we are assuming that we are waiting to send one to initiate a stratumv2
    /// connection.
    pub setup_conn_msg: Option<SetupConnection>,

    /// Outgoing message buffer used to queue messages to be sent to the
    /// counterparty on this connection. This would typically messages queued
    /// by message handlers receiving and processing a message and requiring
    /// to send a response.
    pub pending_msg_buffer: Mutex<Vec<Message>>,
}

impl<S> Peer<S>
where
    S: Session,
{
    pub fn new(encryptor: S) -> Self {
        Peer {
            encryptor,
            setup_conn_msg: None,
            pending_msg_buffer: Mutex::new(Vec::new()),
        }
    }

    /// Drains the Messages from the pending_msg_buffer in order them to be sent
    /// over the wire. An empty buffer is left in it's place.
    pub fn get_pending_msgs(&self) -> Vec<Message> {
        let mut result = Vec::new();
        mem::swap(&mut *self.pending_msg_buffer.lock().unwrap(), &mut result);
        result
    }
}
