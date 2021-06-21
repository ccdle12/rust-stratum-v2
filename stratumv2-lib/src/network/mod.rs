// mod message_handler;

// pub use message_handler::{ConnectionInitiator, ConnectionReceiver};

// Design:
// -> TCPConnection -> Connection{ buffers and tcp conn }
// -> PeerManager::new_inbound(Connection)
//   -> ChannelEncryptor{}
//   -> self.peers.insert(Peer {channel_encryptor, buffers})
//
// -> Connection::schedule_read(peer_manager, conn)
//
// TODO: Make this TDD by using traits
/// The raw connection of a peer.
// NOTE: net_tokio doesn't implement the writer trait so might need to make
// a tokio one Connection object or a network::tokio::Connection
pub struct Connection {
    // writer: TODO: A write trait object?
// TODO:
// - a write to tcp stream
// - a read from tcp stream
}

// impl Connection {
//     fn schedule_read()
// }

/// Contains the state and business logic of each connected peer.
pub struct Peer {
    channel_encryptor: ChannelEncrypter,
    // TODO: Some kind of their flags field?
}

/// Contains all the connected peers.
pub struct PeerManager {
    peers: Vec<Peer>,
    message_handler: MessageHandler,
}

/// Contains the state of the noise encrypted communication.
pub struct ChannelEncrypter {
    // TODO: Noise State?
// TODO: Their static key, pub key? some kind of identifier?
// TODO: Maybe an encrypt/decrypt? maybe already handled in noise
}

/// Generic MessageHandling trait.
pub struct MessageHandler {}

// Trait implementations for certain responsiblity for the MessageHandler to implement
// impl ExtendedMiningMsgHandler for MessageHandler {
//    fn on_open_extended_mining_channel(&self, mesage: &mining:OpenExtendedMiningChannel) -> Result<()> {
//      // do stuff...
//      // Add a message on to a wire, maybe another trait?
//      self.send_message(some_new_message_response)?
//    }

// impl StandardMiningMsgHandler for MessageHandler {
//    fn on_open_extended_mining_channel(&self, mesage: &mining:OpenExtendedMiningChannel) -> Result<()> {
//      // do stuff...
//      // Add a message on to a wire, maybe another trait?
//      self.send_message(some_new_message_response)?
//    }
