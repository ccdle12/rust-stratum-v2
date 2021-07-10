/// Config contains the configuration for a networked device. This maybe
/// separated into Upstream or Downstream configs depending on how each device
/// requirements begin to diverge. Equally this maybe later moved into an upstream
/// networked crate if it makes sense to do so.
#[derive(Clone)]
pub struct Config {
    pub listening_addr: String,
    pub local_network_encryption: bool,
}

impl Config {
    pub fn new(listening_addr: String, local_network_encryption: bool) -> Self {
        Config {
            listening_addr,
            local_network_encryption,
        }
    }
}
