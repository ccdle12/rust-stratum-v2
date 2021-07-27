use rand::rngs::OsRng;
use std::io;
use std::time::SystemTime;
use stratumv2::{
    bitcoin::util::base58,
    codec::{deserialize, serialize, Deserializable, Serializable},
    noise::{
        new_noise_initiator, new_noise_responder, AuthorityKeyPair, AuthorityPublicKey,
        CertificateFormat, NoiseSession, SignatureNoiseMessage, SignedCertificate, StaticKeyPair,
    },
    types::unix_timestamp::{system_unix_time_to_u32, unix_u32_now},
};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

const POOL_ADDR: &str = "127.0.0.1:8085";
const MINER_ADDR: &str = "127.0.0.1:8545";

#[tokio::main]
async fn main() {
    let authority_keypair = AuthorityKeyPair::generate(&mut OsRng {});
    let authority_public_key = base58::encode_slice(&authority_keypair.public.to_bytes());

    tokio::spawn(async move {
        Pool::new(&POOL_ADDR, &authority_keypair).listen().await;
    });

    sleep(Duration::from_secs(2)).await;

    let mut miner = Miner::new(&MINER_ADDR, &authority_public_key);

    miner
        .send_message(
            &TcpStream::connect(&POOL_ADDR).await.unwrap(),
            &mut [0u8; 1024],
        )
        .await;

    miner.listen().await;
}

/// Pool is a convenience struct to demonstrate simple behaviour of a Mining Pool.
struct Pool<'a> {
    /// Listening address of the Mining Pool to accept incoming connections.
    listening_addr: &'a str,
    authority_keypair: &'a AuthorityKeyPair,
    // TODO: Pass the static keypair on constructor as option
    static_keypair: StaticKeyPair,
    noise_session: NoiseSession,
}

impl<'a> Pool<'a> {
    fn new(listening_addr: &'a str, authority_keypair: &'a AuthorityKeyPair) -> Pool<'a> {
        let static_keypair = StaticKeyPair::default();

        Pool {
            listening_addr,
            authority_keypair,
            static_keypair: static_keypair.clone(),
            noise_session: new_noise_responder(Some(static_keypair)),
        }
    }

    /// Listen on the port and handle the messages.
    async fn listen(&mut self) {
        let listener = TcpListener::bind(&self.listening_addr).await.unwrap();
        let mut buffer = [0u8; 1024];

        match listener.accept().await {
            Ok((socket, _)) => loop {
                match socket.try_read(&mut buffer) {
                    Ok(_) => {
                        &self.handle_recv_bytes(&mut buffer).await;
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

    async fn handle_recv_bytes(&mut self, buffer: &mut [u8]) {
        // Receive the noise handshake messages and return.
        self.noise_session.recv_message(buffer).unwrap();
        self.send_message(&TcpStream::connect(&MINER_ADDR).await.unwrap(), buffer)
            .await;

        // Construct and send the SignatureNoiseMessage.
        let valid_from = unix_u32_now().unwrap();
        let not_valid_after =
            system_unix_time_to_u32(&(SystemTime::now() + Duration::from_secs(5))).unwrap();

        let key = self.static_keypair.get_public_key();
        let cert = SignedCertificate::new(0, valid_from, not_valid_after, &key).unwrap();

        let signature_noise_msg =
            SignatureNoiseMessage::from_auth_key(&self.authority_keypair, &cert).unwrap();

        let serialized_msg = serialize(&signature_noise_msg).unwrap();

        let mut buf = [0u8; 1024];
        buf[..serialized_msg.len()].copy_from_slice(&serialized_msg);

        self.send_message(&TcpStream::connect(&MINER_ADDR).await.unwrap(), &mut buf)
            .await;
    }

    // TODO: Update this to use Frameable trait.
    async fn send_message(&mut self, stream: &TcpStream, msg: &mut [u8]) {
        self.noise_session.send_message(msg).unwrap();
        stream.try_write(&msg).unwrap();
    }
}

/// Miner is a convenience struct to demonstrate simple behaviour of a Miner.
struct Miner<'a> {
    /// Listening address of the miner to accept incoming connections.
    listening_addr: &'a str,
    /// Base58 encoded string of the authority public key.
    authority_public_key: &'a str,
    noise_session: NoiseSession,
}

impl<'a> Miner<'a> {
    pub fn new(listening_addr: &'a str, authority_public_key: &'a str) -> Miner<'a> {
        Miner {
            listening_addr,
            authority_public_key,
            noise_session: new_noise_initiator(),
        }
    }

    async fn listen(&mut self) {
        let listener = TcpListener::bind(&self.listening_addr).await.unwrap();
        let mut buffer = [0u8; 1024];

        loop {
            match listener.accept().await {
                Ok((socket, _)) => loop {
                    match socket.try_read(&mut buffer) {
                        Ok(_) => {
                            &self.handle_recv_bytes(&mut buffer).await;
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
    }

    async fn handle_recv_bytes(&mut self, buffer: &mut [u8]) {
        // TODO: Rethink this logic, after calling `recv_message` the state
        // will be transistioned into transport stage but won't be a
        // SignatureNoiseMessage.
        if self.noise_session.is_transport() {
            self.noise_session.recv_message(buffer).unwrap();

            // Deserialize and recreate the certificate format. Validate the
            // signature is valid over the counter parties static key.
            let msg = deserialize::<SignatureNoiseMessage>(buffer).unwrap();
            let remote_static_key = self.noise_session.get_remote_static_public_key().unwrap();

            println!(
                "Is the SignatureNoiseMessage valid? - {:?}",
                CertificateFormat::new(&self.authority_public_key, &remote_static_key, &msg)
                    .unwrap()
                    .verify()
                    .is_ok()
            );
        } else {
            self.noise_session.recv_message(buffer).unwrap();
        }
    }

    // TODO: Update this to use Frameable trait.
    async fn send_message(&mut self, stream: &TcpStream, msg: &mut [u8]) {
        self.noise_session.send_message(msg).unwrap();
        stream.try_write(&msg).unwrap();
    }
}
