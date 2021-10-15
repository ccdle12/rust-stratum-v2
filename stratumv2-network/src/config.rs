use stratumv2_noise::{SignatureNoiseMessage, StaticKeyPair};

/// NoiseConfig contains the configuration for devices to assign a pre-defined
/// StaticKeyPair and SignatureNoiseMessage signed by the Certificate Authority
/// of the Mining Pool. This Config would usually only be used by an Upstream Server
/// (Mining Pool Server).
#[derive(Clone)]
pub struct NoiseConfig {
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

impl NoiseConfig {
    pub fn new(sig_noise_msg: SignatureNoiseMessage, static_key: StaticKeyPair) -> Self {
        NoiseConfig {
            sig_noise_msg,
            static_key,
        }
    }
}

/// NetworkConfig contains the configuration networking for all networked devices.
/// This maybe separated into Upstream or Downstream configs depending on how
/// each device requirements begin to diverge. Equally this maybe later moved
/// into an upstream networked crate if it makes sense to do so.
#[derive(Clone)]
pub struct NetworkConfig {
    /// The public networked listening address of this device.
    pub listening_addr: String,

    /// TODO: A flag determining whether this device will accept insecure communication
    /// on a local network.
    pub local_network_encryption: bool,
}

impl NetworkConfig {
    pub fn new(listening_addr: String, local_network_encryption: bool) -> Self {
        NetworkConfig {
            listening_addr,
            local_network_encryption,
        }
    }
}

/// ServerConfig contains the configurations for state and decision making logic
/// for a Mining Pool Server.
pub struct ServerConfig {
    pub mining_flags: stratumv2_mining::SetupConnectionFlags,
}

impl ServerConfig {
    pub fn new(mining_flags: stratumv2_mining::SetupConnectionFlags) -> Self {
        ServerConfig { mining_flags }
    }
}
