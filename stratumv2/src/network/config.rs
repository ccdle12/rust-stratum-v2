use crate::noise::{SignatureNoiseMessage, StaticKeyPair};

/// Config contains the configuration for a networked device. This maybe
/// separated into Upstream or Downstream configs depending on how each device
/// requirements begin to diverge. Equally this maybe later moved into an upstream
/// networked crate if it makes sense to do so.
#[derive(Clone)]
pub struct Config {
    pub listening_addr: String,
    pub local_network_encryption: bool,
    pub sig_noise_msg: SignatureNoiseMessage,
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
