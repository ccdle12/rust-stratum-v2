use stratumv2_lib::noise::{new_noise_initiator, new_noise_responder, NoiseSession};
use tokio::{io, net::TcpStream, sync::mpsc};
// Design:
// -> TCPConnection -> Connection{ buffers and tcp conn }
// -> PeerManager::new_inbound(Connection)
//   -> ChannelEncryptor{}
//   -> self.peers.insert(Peer {channel_encryptor, buffers})
//
// -> Connection::schedule_read(peer_manager, conn)
//
pub struct Connection<R, W>
where
    R: StreamReader,
    W: StreamWriter,
{
    pub writer: W,
    pub reader: R,
}

impl<R, W> Connection<R, W>
where
    R: StreamReader,
    W: StreamWriter,
{
    pub fn new(writer: W, reader: R) -> Self {
        Connection { writer, reader }
    }
}

// TODO:
impl<R, W> Connection<R, W>
where
    R: StreamReader,
    W: StreamWriter,
{
    async fn schedule_read(&self) {
        println!("asdIOQWERJOEQWIRJQEWRQEWR");
        // TODO:
        // 0. Create a mpsc
        // TODO: It is an arbitrary channel size, need to look into this more
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(1);

        // 1. Spawn a tokio task to read_stream(), send the result of the mpsc
        tokio::spawn(async move {
            // TODO: th
            // let bytes = self.reader.read_stream();
            let _ = tx.send(vec![0x00]).await;
            println!("SENT");
        });

        // 3. Blocking loop read from the rx part of the channel
        // TODO: consider using tokio::select!
        // TODO: consider using a loop {}
        // TODO: This currently does not block
        match rx.recv().await {
            Some(v) => println!("RECEIVED!!!: {:?}", v),
            None => println!("NOTHING RECEIVED!!!"),
        }

        println!("FINISHED");
    }
}

// TODO: Writer Trait
pub trait StreamReader {
    fn read_stream(&self) -> Vec<u8>;
}

pub trait StreamWriter {
    fn write_stream(&self);
}

// TODO: Concrete reader and writer
// TODO: Implement the read to and write to for both TcpStreamWriter and TcpStreamReader
pub struct TcpStreamReader(io::ReadHalf<TcpStream>);

impl StreamReader for TcpStreamReader {
    fn read_stream(&self) -> Vec<u8> {
        println!("not yet implemented");
        vec![0x00]
    }
}

pub struct TcpStreamWriter(io::WriteHalf<TcpStream>);

impl StreamWriter for TcpStreamWriter {
    fn write_stream(&self) {
        println!("not yet implemented");
    }
}

/// Contains the state and business logic of each connected peer.
pub struct Peer {
    channel_encryptor: ChannelEncryptor,
    // TODO: Some kind of their flags field?
}

/// Contains all the connected peers.
pub struct PeerManager {
    peers: Vec<Peer>,
    message_handler: MessageHandler,
}

/// Contains the state of the noise encrypted communication.
// TODO:
// 1. If new_inbound, then we should be the noise_responder?
// 2. If new_outbound, then we should be the noise_initiator?
// 3. encrypt_message(bytes) -> Vec<u8>
// 4. decrypt_message(bytes) -> Vec<u8>
pub struct ChannelEncryptor {
    // TODO: Noise State?
    noise_session: NoiseSession,
    // TODO: Their static key, pub key? some kind of identifier?
}

impl ChannelEncryptor {
    // TODO: Need to implement,
    // None should be a static key of the server
    pub fn new_inbound() -> Self {
        ChannelEncryptor {
            noise_session: new_noise_responder(None),
        }
    }

    pub fn new_outbound() -> Self {
        ChannelEncryptor {
            noise_session: new_noise_initiator(),
        }
    }

    // TODO: Need to implement
    pub fn encrypt_message(bytes: &[u8]) -> Vec<u8> {
        vec![]
    }

    // TODO: Need to implement
    pub fn decrypt_message(bytes: &[u8]) -> Vec<u8> {
        vec![]
    }
}

// TODO: MessageHandler that will implement each trait for certain message handler.
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

// #[cfg(test)]
// mod test {
//     use super::*;

//     // TODO: A potentially temporary test.
//     #[test]
//     fn init_connection() {
//         // The connection would typically be initialized with Writers and Readers
//         // that contain a tokio::io::ReadHalf<TcpStream> and tokio::io::WriteHalf<TcpStream>.
//         struct MockReader();

//         impl StreamReader for MockReader {
//             fn read_stream(&self) -> Vec<u8> {
//                 vec![0x00]
//             }
//         }

//         struct MockWriter();

//         impl StreamWriter for MockWriter {
//             fn write_stream(&self) {}
//         }

//         Connection::new(MockWriter {}, MockReader {});
//     }

//     macro_rules! aw {
//         ($e:expr) => {
//             tokio_test::block_on($e)
//         };
//     }

//     #[test]
//     fn schedule_read() {
//         struct MockReader();

//         impl StreamReader for MockReader {
//             fn read_stream(&self) -> Vec<u8> {
//                 vec![0x00]
//             }
//         }

//         struct MockWriter();

//         impl StreamWriter for MockWriter {
//             fn write_stream(&self) {}
//         }

//         let conn = Connection::new(MockWriter {}, MockReader {});
//         aw!(conn.schedule_read());
//         assert_eq!(1, 2);
//     }
// }
