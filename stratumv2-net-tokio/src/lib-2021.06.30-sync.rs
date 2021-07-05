use stratumv2::network::{ChannelEncryptor, Config};
use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc,
};

use std::io::prelude::{Write, Read};
use std::sync::Mutex;

pub struct PeerManager {
    pub peers: Mutex<HashMap<u32, Peer>>,
    // TODO: message handlers
}

impl PeerManager {
    pub fn new() -> Self {
        PeerManager {
            peers: Mutex::new(HashMap::new()),
        }
    
    }

}


// TODO: State about a channel?
pub struct ChannelManager {
    pending_messages: Vec<u8>,
}

// pub struct Peer {
    // pending_read_buf: Vec<u8>,
    // pending_write_buf: Vec<u8>,
// }

// Maybe a field that is a T, that implements writeable, this way a standard
// tcp stream can be implemented
// pub struct Connection {}

// implements a message handler traits?
pub struct MiningReceiverMsgHandler {
    pending_messages: Mutex<Vec<(u32, u8)>>, // for now
}

// pub struct MessageHandler<M: MiningReceiver> {
    // mining_recv_handler: M
// }
// Plan:
//          conn
// client --------> bind, loop { accept }
//                       |
//                       |
//                       V
//                 setup_inbound conn:
//                  - Create a Connection { stream }
//                  - Create Peer { pending_read_buf, pending_write_buf }
//                  - Add Peer to PeerManager, peermanager.add_peer
//                  - Create a MiningConnHandler { pending_messages } and implements the mining
//                  conn handler trait
//                  - Create a MessageHandler { MiningConnHandler, JNConnHandler }
//                    - Each handler will write back out to the pending_messages
//                    - pending_messages will be a Mutex<Vec<(channel_id/node_id, u8)>> for now
//


pub struct Connection<W, R> {
    // addr: SocketAddr,
    // stream: T,
    pub write_stream: W,
    pub read_stream: R
}

impl<W, R> Connection<W, R> where W: Write, R: Read {
    // pub fn new(addr: SocketAddr, stream: TcpStream, rx_msg: RX, rx_err: RX) -> Self {
    pub fn new(write_stream: W, read_stream: R) -> Self {
        Connection {
            write_stream,
            read_stream
        }
    }
}

/// Contains the state and business logic of each connected peer.
pub struct Peer {
    // TODO: Could collapse into one channel and send an Enum { Ok(b), Err(e) } and then
    // match, if Err, shutdown
    // tx_msg: TX,
    // tx_err: TX,
    // channel_encryptor: ChannelEncryptor, // Maybe should be on channel, not for each peer?
    pending_read_buf: Vec<u8>,
    pending_write_buf: Vec<u8>,
    // TODO: Some kind of their flags field?
}

impl Peer {
    // pub fn new(tx_msg: TX, tx_err: TX, channel_encryptor: ChannelEncryptor) -> Self {
    // pub fn new(channel_encryptor: ChannelEncryptor) -> Self {
    pub fn new() -> Self {
        Peer {
            pending_read_buf: vec![],
            pending_write_buf: vec![],
        }
    }

}

use std::net;
use std::sync::Arc;

async fn setup_inbound(stream: net::TcpStream, peer_manager: Arc<PeerManager>, config: &Config) {
    println!("setup called");
    // TODO: Split the stream into a read/write half.
    let write_stream = stream.try_clone().unwrap();
    let read_stream = stream;

    // TODO: Create a Connection.
    let mut conn = Connection::new(write_stream, read_stream);
    
    // TODO: Create a new peer
    let peer = Peer::new();

    // TODO: Pass in the PeerManager
    { // NOTE: Should drop the lock sooner if not using it later.
        let mut peers = peer_manager.peers.lock().unwrap();
        let new_chan_id = 0; // TODO: Maybe this should be looked up to see if, it clashes???
        peers.insert(new_chan_id, peer);
    }

    // TODO: Loop and  read off the stream
    let mut buf = vec![];
    println!("inside of loop in setup inbound, waiting for conn to send a message...");
    match conn.read_stream.read(&mut buf) { // NOTE: This seems to be blocking but no msg can arrive???
        Ok(_v) => println!("received"),
        Err(_) => println!("error"),
    }
    println!("exits");
}

// async schedule_read(conn: &Connection, peer_manager: Arc<PeerManager>, )

// TODO: Maybe I just need to switch back to using tokio everywhere but only
// functionally test the message handlers, the message handlers can still
// add things back to the buffers
#[cfg(test)]
mod test {
    use super::*;
    use tokio::net::{TcpListener, TcpStream};
    use std::{thread, time};

    #[tokio::test]
    async fn naive_test() {
        // 1. Start with a naive networking just so we can assert it does work 
        // networked
        let addr = "127.0.0.1:8000".to_string();
        let config = Config::new(addr.clone(), false);

        let listener = TcpListener::bind(&config.listening_addr).await.unwrap();

        let peer_manager = Arc::new(PeerManager::new());

        // TODO: Turn into std tcp listener and split in half to Write and Read
        // NOTE: THis is the concurrent thread for each connection, it can block
        // in the task if setup_inbound is reached.
        tokio::spawn(async move {
            let std_stream = listener.into_std().unwrap();

            println!("waiting to accept conn");
            // TODO: Usually this would be in a blocking loop
            match std_stream.accept() {
                Ok((socket, addr)) => {
                    println!("received a conn: {:?}", addr);
                    setup_inbound(socket, peer_manager.clone(), &config).await;
                },
                Err(_) => ()
            }
        });
        


        let mut client = TcpStream::connect(addr).await.unwrap();
        // let ten_millis = time::Duration::from_millis(1000);
        // thread::sleep(ten_millis);

        println!("writing"); // NOTE: Why is this never being reached???
        client.try_write(&[0x00]).unwrap();

        assert_eq!(1, 2);
    }
}
