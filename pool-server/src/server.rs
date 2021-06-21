<<<<<<< Updated upstream
=======
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
>>>>>>> Stashed changes
use stratumv2_lib::{
    noise::{AuthorityKeyPair, SignatureNoiseMessage, SignedCertificate, StaticKeyPair},
    types::{unix_u32_now, unix_u32_one_year_from_now},
};

// TODO:
pub struct PoolServer<'a> {
    // TODO:
    pub(crate) authority_keypair: Option<&'a AuthorityKeyPair>,

    // TODO:
    pub(crate) static_keypair: &'a StaticKeyPair,

    // TODO:
    pub(crate) signature_noise_message: Option<SignatureNoiseMessage>,
<<<<<<< Updated upstream
=======

    // TODO: Add the local listening address
    pub(crate) local_addr: &'a str,
>>>>>>> Stashed changes
}

impl<'a> PoolServer<'a> {
    // TODO:
    pub fn new(
        authority_keypair: Option<&'a AuthorityKeyPair>,
        static_keypair: &'a StaticKeyPair,
<<<<<<< Updated upstream
=======
        local_addr: &'a str,
>>>>>>> Stashed changes
    ) -> PoolServer<'a> {
        // TODO: Handle these errors
        // NOTE: For now generate the SignedCertificate using default values.
        let signed_cert = SignedCertificate::new(
            0,
            unix_u32_now().unwrap(),
            unix_u32_one_year_from_now().unwrap(),
            &static_keypair.public_key,
        )
        .unwrap();

        // NOTE: I'm not sure what would be the default behaviour but for now,
        // generate a SignatureNoiseMessage that can be sent to clients if
        // the Server loads the AuthorityKeypair into the Server.
        let signature_noise_message = match &authority_keypair {
            Some(a) => Some(SignatureNoiseMessage::from_auth_key(a, &signed_cert).unwrap()),
            _ => None,
        };

        PoolServer {
            authority_keypair,
            static_keypair,
            signature_noise_message,
<<<<<<< Updated upstream
=======
            local_addr,
>>>>>>> Stashed changes
        }
    }

    // TODO:
<<<<<<< Updated upstream
    pub fn listen(self) {
        // TODO:
        loop {
            println!("foo");
=======
    pub fn run(&self) {
        // TODO:
        loop {
            let listener = TcpListener::bind(self.local_addr).unwrap();

            let (stream, socket_addr) = listener.accept().unwrap();

            // 1. Record the incoming stream? - save it to a list?
            let peer = ChannelPeer {
                socket: stream,
                encryption_state: false,
            };

            // TODO: Do the ChannelPeers need to be passed to an executor/runtime
            // to read off the socket if anything exists?
>>>>>>> Stashed changes
        }
    }
}

<<<<<<< Updated upstream
=======
pub struct ChannelPeer {
    /// The local socket for the remote channel peer.
    socket: TcpStream,
    // TODO: Generate a channel ID?
    // TODO: Enum state for the current state of ChannelPeer or simpler to start with flags and then refactor?
    /// TMP: has encrypted communication has been setup
    encryption_state: bool,
}

>>>>>>> Stashed changes
#[cfg(test)]
mod test {
    use super::*;
    use stratumv2_lib::noise::generate_authority_keypair;

    #[test]
    fn init_pool_server() {
        let authority_keypair = generate_authority_keypair();
        let static_keypair = StaticKeyPair::default();

<<<<<<< Updated upstream
        let pool_server = PoolServer::new(Some(&authority_keypair), &static_keypair);
=======
        let pool_server =
            PoolServer::new(Some(&authority_keypair), &static_keypair, "127.0.0.1:8545");
>>>>>>> Stashed changes

        assert!(pool_server.signature_noise_message.is_some());
        assert!(pool_server.authority_keypair.is_some());
    }
<<<<<<< Updated upstream
=======

    #[test]
    fn tmp_test() {
        let authority_keypair = generate_authority_keypair();
        let static_keypair = StaticKeyPair::default();

        let pool_server =
            PoolServer::new(Some(&authority_keypair), &static_keypair, "127.0.0.1:8545");

        pool_server.run();
    }
>>>>>>> Stashed changes
}
