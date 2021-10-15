use crate::{
    error::Result as NoiseResult,
    types::{StaticKey, StaticKeyPair, StaticPublicKey},
};
use noiseexplorer_nx::{noisesession::NoiseSession as NxNoiseSession, types::Keypair as NxKeypair};

pub trait Session {
    fn new_noise_responder(static_keypair: Option<StaticKeyPair>) -> Self;
    fn new_noise_initiator() -> Self;
    fn recv_message(&mut self, buf: &mut [u8]) -> NoiseResult<()>;
    fn send_message(&mut self, buf: &mut [u8]) -> NoiseResult<()>;
    fn is_transport(&self) -> bool;
    fn get_handshake_hash(&self) -> Option<[u8; 32]>;
    fn get_remote_static_public_key(&self) -> Option<StaticPublicKey>;
}

pub struct NoiseSession {
    inner: NxNoiseSession,
}

impl Session for NoiseSession {
    fn new_noise_responder(static_keypair: Option<StaticKeyPair>) -> NoiseSession {
        let key = match static_keypair {
            Some(k) => k,
            None => StaticKeyPair::default(),
        };

        NoiseSession {
            inner: NxNoiseSession::init_session(false, &[], key.get_inner()),
        }
    }

    fn new_noise_initiator() -> NoiseSession {
        NoiseSession {
            inner: NxNoiseSession::init_session(true, &[], NxKeypair::default()),
        }
    }

    fn recv_message(&mut self, buf: &mut [u8]) -> NoiseResult<()> {
        Ok(self.inner.recv_message(buf)?)
    }

    fn send_message(&mut self, buf: &mut [u8]) -> NoiseResult<()> {
        Ok(self.inner.send_message(buf)?)
    }

    fn is_transport(&self) -> bool {
        self.inner.is_transport()
    }

    fn get_handshake_hash(&self) -> Option<[u8; 32]> {
        self.inner.get_handshake_hash()
    }

    fn get_remote_static_public_key(&self) -> Option<StaticPublicKey> {
        match self.inner.get_remote_static_public_key() {
            Some(k) => Some(StaticPublicKey::new(k)),
            None => None,
        }
    }
}
