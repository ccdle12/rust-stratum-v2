use std::io;
use stratumv2_lib::{
    common::SetupConnection,
    frame::{frame, unframe, Frameable, Message},
    mining::{SetupConnectionFlags, SetupConnectionSuccess, SetupConnectionSuccessFlags},
    parse::{deserialize, serialize},
    types::MessageType,
};
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

    let setup_conn = SetupConnection::new_mining(
        2,
        2,
        SetupConnectionFlags::REQUIRES_STANDARD_JOBS,
        "0.0.0.0",
        8545,
        "Bitmain",
        "S9i 13.5",
        "braiins-os-2018-09-22-1-hash",
        "some-uuid",
    )
    .unwrap();

    miner
        .send_message(&TcpStream::connect(&POOL_ADDR).await.unwrap(), &setup_conn)
        .await;

    miner.listen().await;
}

/// Pool is a convenience struct to demonstrate simple behaviour of a Mining Pool.
struct Pool<'a> {
    /// Listening address of the Mining Pool to accept incoming connections.
    listening_addr: &'a str,

    /// The required feature flags for the mining sub protocol. These flags
    /// should be sent on a SetupConnectionSuccess.
    required_mining_feature_flags: SetupConnectionSuccessFlags,
}

impl<'a> Pool<'a> {
    fn new(listening_addr: &'a str) -> Pool<'a> {
        Pool {
            listening_addr,
            required_mining_feature_flags: SetupConnectionSuccessFlags::REQUIRES_FIXED_VERSION,
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
        let network_message = deserialize::<Message>(&buffer).unwrap();

        match network_message.message_type {
            MessageType::SetupConnection => {
                let setup_conn = unframe::<SetupConnection>(&network_message).unwrap();

                match setup_conn {
                    SetupConnection::Mining(v) => {
                        let conn_success = SetupConnectionSuccess::new(
                            v.min_version,
                            self.required_mining_feature_flags,
                        )
                        .unwrap();

                        println!("Pool: sending SetupConnectionSuccess message");
                        let network_message = frame(&conn_success).unwrap();
                        let buffer = serialize(&network_message).unwrap();

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

    async fn send_message<T: Frameable>(&self, stream: &TcpStream, msg: &T) {
        let network_message = frame(msg).unwrap();
        let buffer = serialize(&network_message).unwrap();

        stream.try_write(&buffer).unwrap();
    }

    async fn handle_recv_bytes(&self, buffer: &[u8]) {
        let network_message = deserialize::<Message>(&buffer).unwrap();

        match network_message.message_type {
            MessageType::SetupConnectionSuccess => {
                let setup_conn_success =
                    unframe::<SetupConnectionSuccess>(&network_message).unwrap();

                println!("Miner: Received a SetupConnectionSuccess message with feature flags supported by the Mining Pool: {:?}", setup_conn_success.flags)
            }
            _ => (),
        }
    }
}
