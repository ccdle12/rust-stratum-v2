use std::borrow::Cow;
use std::io;
use stratumv2::common::NetworkFrame;
use stratumv2::mining;
use stratumv2::types::MessageTypes;
use stratumv2::util::frame;
use stratumv2::{Deserializable, Framable, Protocol};
use tokio::net::{TcpListener, TcpStream};

// Addreses and ports for the example.
const POOL_ADDR: &str = "127.0.0.1:8080";
const MINER_ADDR: &str = "127.0.0.1:8545";

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        println!("Pool: mining pool now listening for connections");
        Pool::new(&POOL_ADDR).listen().await;
    });

    println!("Miner: sending SetupConnection for new Mining Connection");
    let miner = Miner::new(&MINER_ADDR);

    let setup_connection_msg = mining::SetupConnection::new(
        2,
        2,
        Cow::Borrowed(&[mining::SetupConnectionFlags::RequiresStandardJobs]),
        "0.0.0.0",
        8545,
        "Bitmain",
        "S9i 13.5",
        "braiins-os-2018-09-22-1-hash",
        "some-uuid",
    )
    .unwrap();

    miner
        .send_message(
            &TcpStream::connect(&POOL_ADDR).await.unwrap(),
            setup_connection_msg,
        )
        .await;

    miner.listen().await;
}

/// Pool is a convenience struct to demonstrate simple behaviour of a Mining Pool.
struct Pool<'a> {
    /// Listening address of the Mining Pool to accept incoming connections.
    listening_addr: &'a str,

    /// The required feature flags for the mining sub protocol. These flags
    /// should be sent on a SetupConnectionSuccess.
    required_mining_feature_flags: &'a [mining::SetupConnectionSuccessFlags],
}

impl<'a> Pool<'a> {
    fn new(listening_addr: &'a str) -> Pool<'a> {
        Pool {
            listening_addr,
            required_mining_feature_flags: &[
                mining::SetupConnectionSuccessFlags::RequiresFixedVersion,
            ],
        }
    }

    /// Listen on the port and handle the messages.
    async fn listen(&self) {
        let listener = TcpListener::bind(&self.listening_addr).await.unwrap();
        let mut buffer = [0u8; 1024];

        match listener.accept().await {
            Ok((socket, _)) => loop {
                match socket.try_read(&mut buffer) {
                    Ok(_) => {
                        &self.handle_recv_bytes(&buffer).await;
                        break;
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    _ => continue,
                }
            },
            Err(e) => println!("failed to accept client {:?}", e),
        }
    }

    async fn handle_recv_bytes(&self, buffer: &[u8]) {
        let network_frame = NetworkFrame::deserialize(&buffer).unwrap();

        match network_frame.msg_type {
            MessageTypes::SetupConnection => {
                // The first byte in a SetupConnection message defines the
                // protocol.
                match Protocol::from(network_frame.payload[0]) {
                    Protocol::Mining => {
                        let setup_conn =
                            mining::SetupConnection::deserialize(&network_frame.payload).unwrap();

                        let conn_success = mining::SetupConnectionSuccess::new(
                            setup_conn.min_version,
                            Cow::Borrowed(self.required_mining_feature_flags),
                        );

                        println!("Pool: sending SetupConnectionSuccess message");
                        let buffer = frame(conn_success).unwrap();

                        TcpStream::connect(&MINER_ADDR)
                            .await
                            .unwrap()
                            .try_write(&buffer)
                            .unwrap();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

/// Miner is a convenience struct to demonstrate simple behaviour of a Miner.
struct Miner<'a> {
    /// Listening address of the miner to accept incoming connections.
    listening_addr: &'a str,
}

impl<'a> Miner<'a> {
    fn new(listening_addr: &'a str) -> Miner<'a> {
        Miner { listening_addr }
    }

    async fn listen(&self) {
        let listener = TcpListener::bind(&self.listening_addr).await.unwrap();
        let mut buffer = [0u8; 1024];

        match listener.accept().await {
            Ok((socket, _)) => loop {
                match socket.try_read(&mut buffer) {
                    Ok(_) => {
                        println!("Miner: received message from Pool");
                        &self.handle_recv_bytes(&buffer).await;
                        break;
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    _ => continue,
                }
            },
            Err(e) => println!("failed to accept client {:?}", e),
        }
    }

    async fn send_message<T: Framable>(&self, stream: &TcpStream, msg: T) {
        let buffer = frame(msg).unwrap();
        stream.try_write(&buffer).unwrap();
    }

    async fn handle_recv_bytes(&self, buffer: &[u8]) {
        let network_frame = NetworkFrame::deserialize(&buffer).unwrap();

        match network_frame.msg_type {
            MessageTypes::SetupConnectionSuccess => {
                let setup_conn_success =
                    mining::SetupConnectionSuccess::deserialize(&network_frame.payload).unwrap();

                println!("Miner: Received a SetupConnectionSuccess message with feature flags supported by the Mining Pool: {:?}", setup_conn_success.flags)
            }
            _ => (),
        }
    }
}
