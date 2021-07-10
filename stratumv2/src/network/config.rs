use crate::noise::{SignatureNoiseMessage, StaticKeyPair};

/// Config contains the configuration for a networked device. This maybe
/// separated into Upstream or Downstream configs depending on how each device
/// requirements begin to diverge. Equally this maybe later moved into an upstream
/// networked crate if it makes sense to do so.
#[derive(Clone)]
pub struct Config {
    /// The public networked listening address of this device.
    pub listening_addr: String,

    /// TODO: A flag determining whether this device will accept insecure communication
    /// on a local network.
    pub local_network_encryption: bool,

    /// The SignatureNoiseMessage is intended to be read from disk so that a
    /// Mining Pool Server can send this message at the end of a noise handshake.
    /// If the SignatureNoiseMessage does not exist on disk, the intention is
    /// that after generating it for the first time, it will be stored in the
    /// datadir of the device.
    pub sig_noise_msg: SignatureNoiseMessage,

    /// The StaticKeyPair is the keypair used by the Upstream Device
    /// (Mining Pool Server or Mining Proxy) for all noise sessions. If the
    /// Upstream Device is a Mining Pool Server, then the StaticKeyPair will
    /// be used to generate the SignatureNoiseMessage. The intention is for
    /// the StaticKeyPair to be read from disk. If it is not available on disk,
    /// then after generating it for the first time, it will be persisted
    /// in the datadir of the device.
    pub static_key: StaticKeyPair,
}

impl Config {
    pub fn new(
        listening_addr: String,
        local_network_encryption: bool,
        sig_noise_msg: SignatureNoiseMessage,
        static_key: StaticKeyPair,
    ) -> Self {
        Config {
            listening_addr,
            local_network_encryption,
            sig_noise_msg,
            static_key,
        }
    }
}
