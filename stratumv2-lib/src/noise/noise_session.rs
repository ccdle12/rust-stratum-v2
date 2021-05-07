use crate::noise::types::StaticKeyPair;
use noiseexplorer_nx::types::Keypair;

/// NoiseSession is a struct that contains all the state required to handle a
/// key exchange and subsequent encrypted communication.
pub type NoiseSession = noiseexplorer_nx::noisesession::NoiseSession;

/// Creates a NoiseSession for a responder, this will be the Upstream Node (Server)
/// with the option of using a pre-determined StaticKeyPair.
pub fn new_noise_responder(static_keypair: Option<StaticKeyPair>) -> NoiseSession {
    let key = match static_keypair {
        Some(k) => k,
        None => Keypair::default(),
    };

    NoiseSession::init_session(false, &[], key)
}

/// Creates a NoiseSession for an initiator, this will be the Downstream Node (Client).
pub fn new_noise_initiator() -> NoiseSession {
    NoiseSession::init_session(true, &[], Keypair::default())
}
