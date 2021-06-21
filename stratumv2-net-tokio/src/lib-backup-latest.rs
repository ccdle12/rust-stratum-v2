use stratumv2_lib::noise::{new_noise_initiator, new_noise_responder, NoiseSession};
use tokio::{
    io,
    io::{AsyncReadExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc,
};

// Design:
// -> TCPConnection -> Connection{ buffers and tcp conn }
// -> PeerManager::new_inbound(Connection)
//   -> ChannelEncryptor{}
//   -> self.peers.insert(Peer {channel_encryptor, buffers})
//
// -> Connection::schedule_read(peer_manager, conn)
//

// pub struct Connection {
// pub writer: WriteHalf<TcpStream>,
// pub reader: ReadHalf<TcpStream>,
// }

//
pub struct Connection {}

// TODO:
impl Connection {
    async fn schedule_read(mut reader: ReadHalf<TcpStream>) {
        println!("asdIOQWERJOEQWIRJQEWRQEWR");
        // TODO:
        // 0. Create a mpsc
        // TODO: It is an arbitrary channel size, need to look into this more
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(1);

        let mut buf = [0; 8192];

        // 1. Spawn a tokio task to read_stream(), send the result of the mpsc
        tokio::spawn(async move {
            // TODO: Why doesn't this block within tokio spawn?
            match reader.read(&mut buf).await {
                Ok(_) => {
                    tx.send(buf.clone().to_vec()).await;
                    ()
                }
                Err(_) => (),
                // let _ = tx.send(buf.clone().to_vec()).await;
            };
            // println!("{:?}", buf);
            // let _ = tx.send(buf.clone().to_vec()).await;
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

#[cfg(test)]
mod test {
    use super::*;
    use tokio::{net::TcpListener, test};

    // TODO: Move this to integration level tests folder and start testing for code paths by
    // sending bytes over the connection
    #[tokio::test]
    async fn naive_connection_test() {
        let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
        TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();

        let stream = listener.accept().await.unwrap().0;
        let (reader, writer) = io::split(TcpStream::from_std(stream.into_std().unwrap()).unwrap());
        Connection::schedule_read(reader).await;

        // TODO:
        // 1. Listen for TCP connections
        // 2. Accept the connection
        // 3. Pass the stream and address to a function to:
        //   - Initialize a Peer { socket, RX }
        // 4. Run a blocking loop and use tokio::select!
        //   - Read from peer rx to send them a message
        //   - Read from tcp stream if available and handle it via a message handler?
    }
}
