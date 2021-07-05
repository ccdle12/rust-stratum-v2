// TODO:
// According to Braiins OS: https://github.com/braiins/braiins/blob/d3608188a3e5ac5d7ad6f32c57dcb71536315208/open/protocols/stratum/sim/sim_primitives/stratum_v2/pool.py#L89
//   - A ChannelRegistry contains a collection of channels linked to a unique connection id,
//   - Connection is 1 to Many Channels
//
//  - Think through how to remodel this so that any manager contains 1 connection -> many channels
//
//  - We need to have:
//    - Connection ->-> Vec<Channels>
//
// TODO:
// - The NODE MUST send a SetupConnection first, so we need to retain state
// on the node, asking if we have received a SetupConnection, if we don't then
// we have to close the channel (maybe).
//
// - We also need to do the NoiseHandshake before setting up a SetupConnection
// or setting up a Channel. So we should have the channel encryptor state on the
// Peer and before writing out to the Connection, we should encrypt and decrypt here.
//
//
// TODO: Reasses the connection logic:
//
// 1. Client -> Connects
// 2. Server: Create a Connection that holds network and mpsc chans
// 2a. Server: Create the PeerState { channel_encryptor, setup_conn_msg: None}
//     - The logic for ordering of received msgs:
//       - Expect handshake messaging
//       - Is noise session in transport mode?
//       - Is setup_conn_msg None?
//         - Check that the first message is a setup_conn_msg and update the Peer state
//
// 3. Create the managers to allow a quick time complexity to find a channel and find a peer.
//
//    a. Be able to receive multiple channel opens on the same connection
//      - Receive an open channel msg
//      - Look up the collection of channels according to peer, create channel and add it to the
//      collection
//
//    b. Server: Send a message directly to a certain channel
//       - Look up the channel by ID to update state?
//       - Look up the Peer/Conn using the channel ID
//       - Send msg to Peer/Conn using the message
//
//    DRAFT:
//    - Map{ ConnID, Vec<Channel>} // Lookup channels according to peer connection
//    - pending_msg_event.push((msg, conn_id))
//
//
// 4. Read from the Connection read stream
//   - Check the Peer associated with the Connection, is the Encryption handhshake complete? if not
//   continue the handshake
//   - If the handshake is complete, handle the message
//   - Does the Peer contain a SetupConnection? If not, the message MUST be a SetupConnection, if
//   not disconnect
//     - Remove the and delete the Peer, Shutdown the Stream, Delete the Connection
use stratumv2::common::SetupConnection;
use stratumv2::error::{Error, Result};
use stratumv2::frame::{frame, unframe, Message};
use stratumv2::network::{Channel, Config, ConnectionEncryptor, Encryptor, Peer};
use stratumv2::noise::{SignatureNoiseMessage, StaticKeyPair};
use stratumv2::parse::{deserialize, serialize, Deserializable};
use stratumv2::types::MessageType;

use std::collections::HashMap;
use std::mem;
use std::net::SocketAddr;
use std::sync;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::{
    io,
    io::{AsyncReadExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::{mpsc, Mutex},
};

type ConnID = u32;

/// Contains all the connected peers.
// TODO: Maybe rename this to some else, since its going to contain a ChannelManager,
// PeerManager and maybe MessageHandlers.
// This should be at the network level, NOT the library level. We can just do
// functional tests using the ChannelManager
// pub struct PeerManager {
// NOTE: The manager may receive a message with a certain channel id. How do we
// make sure we can find the right channel to update the state?
pub struct ConnectionManager<E>
where
    E: Encryptor,
{
    /// Contains a protected map of peers linked to a channel id.
    // TODO: 1.
    // - When receiving a new connection, create the object and assign an ID or maybe just a
    // HashSet?
    pub conns: tokio::sync::Mutex<HashMap<ConnID, (Peer<E>, Connection)>>,

    /// Contains the object that tracks and manages the collection of active channels.
    // TODO: 2.
    // - When receiving a message on an existing Connection, we'll call the channel manager
    // to handle the message?
    // NOTE:
    // So maybe when receive a message or the msg handler to needs to perform
    // some kind of state update, we look up the Channel by the ConnID and then
    // find the Channel to make the update to that state?
    pub channel_manager: ChannelManager,

    // NOTE: Should be the message handler that will call internal handlers
    // according to the message received.
    pub msg_handler: MessageHandler,
}

impl<E> ConnectionManager<E>
where
    E: Encryptor,
{
    pub fn new() -> Self {
        ConnectionManager {
            conns: Mutex::new(HashMap::new()),
            channel_manager: ChannelManager::new(),
            msg_handler: MessageHandler::new(),
        }
    }
}

// TODO: State about a channel?
/// Channels are different to Peers, because many Peers maybe related to a Channel.
pub struct ChannelManager {
    /// Contains all the channels that belong to a certain connection according
    /// to Connection ID.
    // TODO: 3.
    // - Receive a message on an existing connection, use the connection ID
    // to find the channels associated with it, if the message is for a certain
    // channel, look up the channel,
    channels: std::sync::Mutex<HashMap<ConnID, Vec<Channel>>>,
}

impl ChannelManager {
    pub fn new() -> Self {
        ChannelManager {
            channels: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

// TODO: Start implementing trait implementations according to the device.
pub struct MessageHandler {}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {}
    }

    // TODO: Extract this to a common trait since its synchronous
    pub fn handle<E: Encryptor>(&self, msg: &[u8], peer: &mut Peer<E>) -> Result<()> {
        println!("IN HANDLE: {:?}", &msg);
        // TODO:
        // 1. Deframe
        let deframed = deserialize::<Message>(&msg)?;

        match deframed.message_type {
            MessageType::SetupConnection => {
                let setup_conn = unframe::<SetupConnection>(&deframed)?;

                // TODO: Pass the deframed message and Peer to an handler
                // for new connection.
                match &setup_conn {
                    SetupConnection::Mining(m) => {
                        println!("RECEIVED SETUPCONN: {:?}", m.min_version);

                        // TODO: Need to check the supported feature flags of
                        // the server.
                        //
                        // TODO: Need to check the protocol version supported
                        // by the server.
                        //
                        // TODO: This should be the last step.
                        peer.setup_conn_msg = Some(setup_conn);
                    }
                    _ => println!("moop"),
                }
            }
            _ => (()),
        }
        Ok(())
    }
}

/// Contains the state of the noise encrypted communication.

// TODO: Doc strings
pub type TX = mpsc::Sender<Vec<u8>>;
pub type TX_ERR = mpsc::Sender<u8>;

// NOTE: This should NOT be at the library level.
pub struct Connection {
    id: u32,
    addr: SocketAddr,
    tx_msg: TX,
    tx_err: TX_ERR,
}

// TODO: Lets just leave the channel encryptor here.
impl Connection {
    pub fn new(id: u32, addr: SocketAddr, tx_msg: TX, tx_err: TX_ERR) -> Self {
        Connection {
            id,
            addr,
            tx_msg,
            tx_err,
        }
    }
}

async fn process_inbound(
    stream: TcpStream,
    addr: SocketAddr,
    conn_manager: Arc<ConnectionManager<ConnectionEncryptor>>,
    config: &Config,
) {
    // TMP:
    // NOTE: Bounded channel of 100 is arbitrary.
    let mut stream = stream;
    let (tx_msg, mut rx_msg) = mpsc::channel::<Vec<u8>>(100);
    let (tx_err, mut rx_err) = mpsc::channel::<u8>(100);
    // TMP:
    //
    let conn_id = 0; // TODO: This should not clash with a PeerManager?
    let conn = Connection::new(conn_id, addr, tx_msg, tx_err);

    // TMP: This should be in a config, maybe guarded by a Mutex? maybe for now don't
    // but leave a comment to guard it by a Mutex. The same StaticKeyPair is used
    // for all connections since they need to validated by the SignatureNoiseMessage
    // let static_key = StaticKeyPair::default();
    // TMP:

    let encryptor = ConnectionEncryptor::new_inbound(Some(config.static_key.clone()));
    let peer = Peer::new(encryptor);

    {
        // TODO: What if a node is connecting on an existing channel? This would be ok since
        // a mining proxy will have a bunch of downstream nodes on the same channel
        //
        // TODO: How do we know if a new node is connecting to an existing channel
        // or connecting as a new channel?, might have to leave this one for future
        // reference until I can understand the requirements better
        let mut peers = conn_manager.conns.lock().await;
        peers.insert(conn_id, (peer, conn));
    }

    // TODO: Maybe then pass below into a schedule_reads?
    // Block and sending/receiving to the peer.
    loop {
        let mut buf = [0; 1024];
        tokio::select! {
            result = stream.read(&mut buf) => match result {
                Ok(_) => {
                    println!("SERVER: Reading from stream");
                    handle_read_stream(&mut buf, conn_manager.clone(), conn_id, &config).await;
                },
                Err(_) => { println!("BREAK"); break}
            },
             result = rx_msg.recv() => {
                 let result = result.unwrap();
                 println!("Sending response: {:?}", &result);
                // TODO: Maybe it needs to be a Vector<[u8]>
                &stream.try_write(&result).unwrap();
            },
            // NOTE: When an error message is ready, send out over this connection
            // but how do I multiplex??
             _ = rx_err.recv() => {
                 // TODO: Kill TCP Connection and remove any stateful info?
                 // TODO: And then kill this spawned task?
                 // TODO: How do we safely kill the tcpstream and then kill the
                 // process
                 // TODO: I think the client needs to handle writing upstream errors.
                 println!("ERROR - SHUTTING DOWN STREAM");
                 stream.shutdown();
                 return;
             }
        }
    }
}

async fn handle_read_stream<E: Encryptor>(
    mut buf: &mut [u8],
    conn_manager: Arc<ConnectionManager<E>>,
    conn_id: ConnID,
    config: &Config,
) {
    // TODO: 1. Call the peer manager, message handler to handle the message in bytes synchronously
    let mut conns = conn_manager.conns.lock().await;

    // TODO: If its not in here, then return an error or exit?
    let (peer, conn) = conns.get_mut(&conn_id).unwrap();

    // TODO: Need to move the peer to a mutable variable in order for it to be mutably
    // borrowed. Need to check why get_mut() doesn't return a variable that can be
    // borrowed mutably.
    let mut peer = peer;

    // NOTE: Getting past this means we're in the handshake state.
    match handle_noise_handshake(&mut buf, &mut peer.encryptor, conn, &config.sig_noise_msg).await {
        Err(_e) => {
            conn.tx_err.send(0).await.unwrap();
            return;
        }
        Ok(false) => {
            return;
        }
        _ => (),
    }

    // TODO: Currently this will disconnect on any handling error, this maybe
    // too harsh. We might need to separate either ignoring minor errors or
    // disconnecting on egregious errors.
    if let Err(_) = conn_manager.msg_handler.handle(&buf, &mut peer) {
        println!("send err");
        conn.tx_err.send(0u8).await.unwrap();
        return;
    };

    // TODO: Move to another function?
    let msg = peer.get_pending_msgs();
    println!("after geting pending msgs");
    if msg.len() > 0 {
        // TODO: Maybe I should serialize it here and encrypt?
        // TODO: Match each message and decide how to handle
        for m in msg {
            match &m.message_type {
                MessageType::SetupConnection => {
                    let response = serialize(&m).unwrap();

                    // TODO: Handle any updates to the Connection? or Peer?
                    // TODO: Encrypt
                    conn.tx_msg.send(response).await.unwrap();
                }
                _ => (),
            }
        }
    }

    // TMP: So we can force a response back
    conn.tx_msg.send(vec![0u8]).await.unwrap();
    println!("msg length is 0");
}

async fn handle_noise_handshake<E: Encryptor>(
    buf: &mut [u8],
    encryptor: &mut E,
    conn: &Connection,
    sig_noise_msg: &SignatureNoiseMessage,
) -> Result<bool> {
    if !encryptor.is_handshake_complete() {
        println!("SERVER: CHANNEL IS NOT ENCRYPTED");
        // TODO: This will return an error if theres an invalid key, maybe disconnect at that point
        if let Err(_) = conn.tx_msg.send(encryptor.recv_handshake(buf)?).await {
            return Err(Error::Unimplemented());
        }

        if let Err(_) = conn.tx_msg.send(serialize(sig_noise_msg)?).await {
            return Err(Error::Unimplemented());
        }

        return Ok(false);
    }

    println!(
        "SERVER: OUTSIDE OF CHANNEL ENCRYPTOR: {:?}",
        encryptor.is_handshake_complete()
    );

    Ok(true)
}

#[cfg(test)]
mod test {
    use super::*;
    use bitcoin::util::base58;
    use stratumv2::common::SetupConnection;
    use stratumv2::frame::frame;
    use stratumv2::mining::SetupConnectionFlags;
    use stratumv2::noise::CertificateFormat;
    use stratumv2::noise::{generate_authority_keypair, SignatureNoiseMessage, SignedCertificate};
    use stratumv2::parse::serialize;
    use stratumv2::types::{unix_u32_now, unix_u32_one_year_from_now};
    use tokio::{net::TcpListener, test};

    // TODO: Move this to integration level tests folder and start testing for code paths by
    // sending bytes over the connection
    // TODO: Maybe just do it here, the handshake tests.
    #[tokio::test]
    async fn naive_connection_test() {
        let addr = "127.0.0.1:8000".to_string();

        // NOTE: This should be on the server setup by generating a SignatureNoiseMessage
        // TODO: Maybe the server needs to:
        // 1. if sig_noise_msg exists on file?
        //      - Read the static key pair on file
        //      - Read the authority key pair on file
        //      - Generate a signature noise message and write to a datadir
        // 2. else
        //      - Read sig from file and set in a Config?
        // TMP:
        // TODO: How to make this more intuitive and easier to generate?
        // - This needs to be made much simpler
        // 1. Rename SignedCertificate to Certificate
        let static_key = StaticKeyPair::default();
        let signed_cert = SignedCertificate::new(
            0,
            unix_u32_now().unwrap(),
            unix_u32_one_year_from_now().unwrap(),
            &static_key.public_key,
        )
        .unwrap();
        let auth_key = generate_authority_keypair();
        let sig_noise_msg = SignatureNoiseMessage::from_auth_key(&auth_key, &signed_cert).unwrap();
        // TMP:

        // TODO: This should be split with UpstreamConfig and DownstreamConfig
        // TODO: Maybe wrap this in an Arc?
        let config = Config::new(addr.clone(), false, sig_noise_msg, static_key.clone());

        // TODO: Extract TcpListener bind + accept + process task into main()
        let listener = TcpListener::bind(&config.listening_addr).await.unwrap(); // TODO: Handle unwrap.

        let mut peer_manager = Arc::new(ConnectionManager::new());
        tokio::spawn(async move {
            let (stream, socket_addr) = listener.accept().await.unwrap(); // TODO: Handle unwrap by ignoring?
            process_inbound(stream, socket_addr, peer_manager.clone(), &config).await;
        });

        // Simulate a downstream client connection and sending a mesage.
        let mut client = TcpStream::connect(&addr).await.unwrap();
        let mut initiator = ConnectionEncryptor::new_outbound();
        let mut buf = initiator.init_handshake().unwrap();
        client.try_write(&buf).unwrap();

        client.read(&mut buf).await;
        initiator.recv_handshake(&mut buf).unwrap();
        println!("BUFFER IN CLIENT 1: {:?}", buf);

        // TODO: Deserialize and assert its valid.
        let mut buf = [0u8; 1024];
        client.read(&mut buf).await;
        println!("BUFFER IN CLIENT 2: {:?}", buf);
        let sig_noise_msg = deserialize::<SignatureNoiseMessage>(&buf).unwrap();

        // TODO: I need to create a pass through method to get the remote static public key
        // let remote_static_key = client.get_remote_static_public_key().unwrap();
        let key = &base58::encode_slice(&auth_key.public.to_bytes());
        let remote_pubkey = initiator.get_remote_pubkey().unwrap();
        let cert = CertificateFormat::new(&key, &remote_pubkey, &sig_noise_msg).unwrap();
        assert!(cert.verify().is_ok());

        println!(
            "Initiator is channel encrypted: {:?}",
            initiator.is_handshake_complete()
        );

        // NOTE: Send a SetupConnection
        let new_conn = SetupConnection::new_mining(
            2,
            2,
            SetupConnectionFlags::REQUIRES_STANDARD_JOBS
                | SetupConnectionFlags::REQUIRES_VERSION_ROLLING,
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-device-uuid",
        )
        .unwrap();

        let serialized = serialize(&frame(&new_conn).unwrap()).unwrap();
        client.try_write(&serialized).unwrap();

        let mut buf = [0u8; 1024];
        client.read(&mut buf).await;
    }
}
