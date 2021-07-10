use crate::{
    error::Result,
    noise::{new_noise_initiator, new_noise_responder, NoiseSession, StaticKeyPair},
};

/// The Encryptor trait can be used to apply a noise framework encryption implementation
/// for a connection.
pub trait Encryptor {
    fn is_handshake_complete(&self) -> bool;
    fn recv_handshake(&mut self, bytes: &mut [u8]) -> Result<Vec<u8>>;
    fn init_handshake(&mut self) -> Result<Vec<u8>>;
    fn encrypt_message(bytes: &[u8]) -> Vec<u8>;
    fn decrypt_message(bytes: &[u8]) -> Vec<u8>;
}

/// ConnectionEncryptor implements Encryptor providing a common interface to
/// to perform the noise handshake and de/encrypt messsages.
pub struct ConnectionEncryptor {
    noise_session: NoiseSession,
}

impl ConnectionEncryptor {
    /// Initialize a ChannelEncryptor as the receiver of an inbound noise handshake
    /// session. This would typically be upstream devices such as Mining Pool Server.
    pub fn new_inbound(static_key: Option<StaticKeyPair>) -> Self {
        ConnectionEncryptor {
            noise_session: new_noise_responder(static_key),
        }
    }

    /// Initialize a ChannelEncryptor as the initiator of an outbound noise handshake.
    /// This would typically be downstream nodes such as Mining Devices or Mining Proxies.
    pub fn new_outbound() -> Self {
        ConnectionEncryptor {
            noise_session: new_noise_initiator(),
        }
    }
}

impl Encryptor for ConnectionEncryptor {
    /// Checks if the noise handshake has completed, meaning the sender and receiver
    /// can communicate securely.
    fn is_handshake_complete(&self) -> bool {
        self.noise_session.is_transport()
    }

    /// Receives bytes and update the noise handshake state. Will also advance
    /// the handshake state and return the bytes required to send back to
    /// the counter-party.
    fn recv_handshake(&mut self, bytes: &mut [u8]) -> Result<Vec<u8>> {
        self.noise_session.recv_message(bytes)?;
        self.noise_session.send_message(bytes)?;

        Ok(bytes.to_vec())
    }

    /// Initialize the handshake state as the initiator. Will return the bytes
    /// required to send to the receiver.
    fn init_handshake(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 1024];
        self.noise_session.send_message(&mut buf)?;

        Ok(buf.to_vec())
    }

    // TODO:
    /// Encrypt an outbound message.
    fn encrypt_message(bytes: &[u8]) -> Vec<u8> {
        vec![]
    }

    // TODO:
    /// Decrypt an inbound message.
    fn decrypt_message(bytes: &[u8]) -> Vec<u8> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_handshake() {
        let mut initiator = ConnectionEncryptor::new_outbound();
        let mut receiver = ConnectionEncryptor::new_inbound(None);

        let mut x = initiator.init_handshake().unwrap();
        let mut y = receiver.recv_handshake(&mut x).unwrap();
        initiator.recv_handshake(&mut y).unwrap();

        assert!(initiator.is_handshake_complete() && receiver.is_handshake_complete());
    }
}
