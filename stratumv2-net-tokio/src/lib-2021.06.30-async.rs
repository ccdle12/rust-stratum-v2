use stratumv2::network::{ChannelEncryptor, Config};
use std::collections::HashMap;
use tokio::{
    io,
    io::{AsyncReadExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::mpsc,
};
use std::sync::Mutex;

/// Contains all the connected peers.
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

pub struct MiningReceiverMsgHandler {
    pending_messages: Mutex<Vec<(u32, u8)>>, // for now
}

/// Contains the state of the noise encrypted communication.
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;

// TODO: Doc strings
pub type TX = mpsc::Sender<Vec<u8>>;
pub type RX = mpsc::Receiver<Vec<u8>>;

// TODO: MessageHandler that will implement each trait for certain message handler.
// pub struct MessageHandler {}

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

pub struct ServerMessageHandler<'a> {
    peer: &'a mut Peer,
}

impl<'a> ServerMessageHandler<'a> {
    fn new(peer: &'a mut Peer) -> Self {
        ServerMessageHandler { peer }
    }

    async fn handle(&mut self, mut buf: &mut [u8]) {
        // TODO: Could be is peer.channel_encryptor some?
        //
        // The default is that channel_encryptor will be Some.
        // If its None, then we just continue as we usually would
        //
        if !self.peer.channel_encryptor.is_channel_encrypted() {
            println!("INSIDE CHANNEL ENCRYPTED");
            // TODO: Read the docs, what is the behaviour of the server? should it
            // disconnect on faulty messages like this?
            if let Err(_e) = self.peer.channel_encryptor.recv_handshake(&mut buf) {
                // TODO: This is tmp, maybe sending an err we just send e?
                //
                println!("SENDING ERROR OVER TX_ERR");
                // TODO: This send fails because of the return
                self.peer.tx_err.send(vec![0x00]).await.unwrap();
                // TODO: Does the break exit tokio::select! or loop {...}?
                return;
            }

            // Send the channel encryptor handshake information to progress the state
            // of the handshake for both client and server.
            println!("SEND OF TX_MSG");
            self.peer
                .tx_msg
                .send(self.peer.channel_encryptor.handshake_buf.clone())
                .await
                .unwrap();
            return;
        }

        // TODO: Deserialize the buf to get the deserialized Frame
        // TODO: Switch on the msg_type and delegate to other message handler
        //
        // TODO: Remember to encrypt the message on the way out. Maybe Channel
        // Encryptor should be at the Connection level, so that handler just
        // does the logic for handling?
    }
}

// NOTE: THis
pub struct Connection {
    addr: SocketAddr,
    stream: TcpStream,
    rx_msg: RX,
    rx_err: RX,
}

impl Connection {
    pub fn new(addr: SocketAddr, stream: TcpStream, rx_msg: RX, rx_err: RX) -> Self {
        Connection {
            addr,
            stream,
            rx_msg,
            rx_err,
        }
    }
}

/// Contains the state and business logic of each connected peer.
pub struct Peer {
    // TODO: Could collapse into one channel and send an Enum { Ok(b), Err(e) } and then
    // match, if Err, shutdown
    tx_msg: TX,
    tx_err: TX,
    channel_encryptor: ChannelEncryptor,
    // TODO: Some kind of their flags field?
}

impl Peer {
    pub fn new(tx_msg: TX, tx_err: TX, channel_encryptor: ChannelEncryptor) -> Self {
        Peer {
            tx_msg,
            tx_err,
            channel_encryptor,
        }
    }
}

async fn process(stream: TcpStream, addr: SocketAddr, config: &Config) {
    // NOTE: Bounded channel of 100 is arbitrary.
    let (tx_msg, rx_msg) = mpsc::channel::<Vec<u8>>(100);
    let (tx_err, rx_err) = mpsc::channel::<Vec<u8>>(100);

    // Connection should be about specific networking logic e.g.
    // - streams, errors over reading or sending over the wire, disconnecting
    //
    // Peer should be about stratumv2 logic e.g.
    // - which flags the peer provides
    // - channel encryption
    //
    let mut conn = Connection::new(addr, stream, rx_msg, rx_err);

    // TODO: Could be conn.is_local_conn()?
    // - if cfg.force_local_conn_encryption
    //
    // By default always have encrypted communication (defensive)
    // NOT encrypted when:
    // - connection is the same local network
    // - the user has opted to say local networks can bypass encryption
    //
    // 1. channel_encryptor = if conn.is_local() && cfg.allow_unencryped_local_conn {
    //        None
    //    } else {
    //        ChannelEncryptor::new_inbound()
    //    }
    let mut peer = Peer::new(tx_msg, tx_err, ChannelEncryptor::new_inbound());
    let mut msg_handler = ServerMessageHandler::new(&mut peer);

    // Block and sending/receiving to the peer.
    loop {
        let mut buf = [0; 1024];
        // TODO: Maybe call a function handle_read_stream(&conn)
        // async handle_read_stream(mut buf: &[u8], handler: &msg_handler, conn: &conn) -> Vec<u8> {
        //    handler.handle(&mut buf);
        //    let msg = handler.channel.get_msg_pending();
        //
        // } 
        tokio::select! {
            result = conn.stream.read(&mut buf) => match result {
                Ok(_) => msg_handler.handle(&mut buf).await,
                Err(_) => { println!("BREAK"); break}
            },
             result = conn.rx_msg.recv() => {
                println!("SENDING MESSAGE OVER STREAM");
                conn.stream.try_write(&result.unwrap()).unwrap();
            },
             result = conn.rx_err.recv() => {
                 // TODO: Kill TCP Connection and remove any stateful info?
                 // TODO: And then kill this spawned task?
                 // TODO: How do we safely kill the tcpstream and then kill the
                 // process
                 // TODO: I think the client needs to handle writing upstream errors.
                 println!("ERROR - SHUTTING DOWN STREAM");
                 conn.stream.shutdown();
                 return;
             }
        }
        println!("OUTSIDE OF TOKIO SELECT BUT STILL IN LOOP");
    }
    // TODO: Maybe tokio select needs to return
    println!("OUTSIDE LOOP AFTER BREAK");
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::{net::TcpListener, test};

    // TODO: Move this to integration level tests folder and start testing for code paths by
    // sending bytes over the connection
    #[tokio::test]
    async fn naive_connection_test() {
        let addr = "127.0.0.1:8000".to_string();
        let config = Config::new(addr.clone(), false);

        // TODO: Extract TcpListener bind + accept + process task into main()
        let listener = TcpListener::bind(&config.listening_addr).await.unwrap(); // TODO: Handle unwrap.

        tokio::spawn(async move {
            let (stream, socket_addr) = listener.accept().await.unwrap(); // TODO: Handle unwrap by ignoring?
            process(stream, socket_addr, &config).await;
        });

        // Simulate a downstream client connection and sending a mesage.
        let mut client = TcpStream::connect(addr).await.unwrap();

        // TODO: This will fail because the client is NOT sending over a public key.
        // The client needs to setup an initiator and send over the bytes generated
        // to start the handshake.
        //
        client.try_write(&[0x00]).unwrap();

        // Block and wait for a response. We can test the codepaths here.
        // This should represents a first message to start the channel encryption.
        // NOTE: This blocks waiting for a response.
        let mut buf = [0; 1024];
        client.read(&mut buf).await;

        // println!("BUFFER: {:?}", buf);
        assert_eq!(1, 2);
    }

    // TODO:
    // Do a test where sending 1 byte should cause the handshake recv to fail
    // async fn handshake_error_test() {}
}
