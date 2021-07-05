use crate::{
    error::Result,
    noise::{new_noise_initiator, new_noise_responder, NoiseSession},
};

/// The Encryptor trait can be used to apply a noise framework encryption implementation
/// over a connection.
pub trait Encryptor {
    fn is_handshake_complete(&self) -> bool;
    fn recv_handshake(&mut self, bytes: &mut [u8]) -> Result<Vec<u8>>;
    fn init_handshake(&mut self) -> Result<Vec<u8>>;
    fn encrypt_message(bytes: &[u8]) -> Vec<u8>;
    fn decrypt_message(bytes: &[u8]) -> Vec<u8>;
}

/// ConnectionEncryptor is a stateful struct used for all devices. It handles
/// and contains the state for a noise handshake and provides an easy interface
/// to encrypt/decrypt messages.
pub struct ConnectionEncryptor {
    noise_session: NoiseSession,
}

impl ConnectionEncryptor {
    // TODO: update new_inbound() to new_inbound(Option<StaticKey>) to allow the
    // caller to read a static key from file.
    /// Initialize a ChannelEncryptor as the receiver of an inbound noise handshake
    /// session. This would typically be upstream devices such as Mining Pool Server.
    pub fn new_inbound() -> Self {
        ConnectionEncryptor {
            noise_session: new_noise_responder(None),
        }
    }

    /// Initialize a ChannelEncryptor as the initiator of an outbound noise handshake.
    /// This would typically be downstream nodes such as Mining Devices or Mining Proxies.
    // pub fn new_initiator() -> Self {
    pub fn new_outbound() -> Self {
        ConnectionEncryptor {
            noise_session: new_noise_initiator(),
        }
    }
}

impl Encryptor for ConnectionEncryptor {
    /// Check if the current state of the encryptor is in post-handshake meaning
    /// the channel is encrypting messages.
    fn is_handshake_complete(&self) -> bool {
        self.noise_session.is_transport()
    }

    /// Receive bytes to update the state of the noise handshake. The last message
    /// is recorded in the handshake_buf.
    fn recv_handshake(&mut self, bytes: &mut [u8]) -> Result<Vec<u8>> {
        self.noise_session.recv_message(bytes)?;
        self.noise_session.send_message(bytes)?;

        Ok(bytes.to_vec())
    }

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

// impl ConnectionEncryptor {
// /// Check if the current state of the encryptor is in post-handshake meaning
// /// the channel is encrypting messages.
// pub fn is_handshake_complete(&self) -> bool {
//     self.noise_session.is_transport()
// }

// /// Receive bytes to update the state of the noise handshake. The last message
// /// is recorded in the handshake_buf.
// pub fn recv_handshake(&mut self, bytes: &mut [u8]) -> Result<Vec<u8>> {
//     self.noise_session.recv_message(bytes)?;
//     self.noise_session.send_message(bytes)?;

//     Ok(bytes.to_vec())
// }

// pub fn init_handshake(&mut self) -> Result<Vec<u8>> {
//     let mut buf = [0u8; 1024];
//     self.noise_session.send_message(&mut buf)?;

//     Ok(buf.to_vec())
// }

// // TODO: update new_inbound() to new_inbound(Option<StaticKey>) to allow the
// // caller to read a static key from file.
// /// Initialize a ChannelEncryptor as the receiver of an inbound noise handshake
// /// session. This would typically be upstream devices such as Mining Pool Server.
// pub fn new_receiver() -> Self {
//     ConnectionEncryptor {
//         noise_session: new_noise_responder(None),
//     }
// }

// /// Initialize a ChannelEncryptor as the initiator of an outbound noise handshake.
// /// This would typically be downstream nodes such as Mining Devices or Mining Proxies.
// pub fn new_initiator() -> Self {
//     ConnectionEncryptor {
//         noise_session: new_noise_initiator(),
//     }
// }

// // TODO:
// /// Encrypt an outbound message.
// pub fn encrypt_message(bytes: &[u8]) -> Vec<u8> {
//     vec![]
// }

// // TODO:
// /// Decrypt an inbound message.
// pub fn decrypt_message(bytes: &[u8]) -> Vec<u8> {
//     vec![]
// }
// }
