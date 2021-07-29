use crate::{
    common::SetupConnection,
    error::Result,
    job_negotiation, mining,
    network::{Encryptor, Peer},
};

/// A trait that should be applied to a message handler for a Mining Pool Server.
pub trait ServerMsgHandler<E: Encryptor> {
    fn handle_new_conn(
        &self,
        server_flags: &mining::SetupConnectionFlags,
        new_conn: SetupConnection,
        peer: &mut Peer<E>,
    ) -> Result<()>;
}

/// A trait that should be applied to downstream devices such as Mining Devices
/// and proxies that can handle responses after attempting to open a New Mining
/// Connection.
pub trait MiningInitiator {
    fn handle_mining_conn_success(conn_success: mining::SetupConnectionSuccess);
    fn handle_mining_conn_error(conn_error: mining::SetupConnectionError);
}

/// A trait that should be applied to downstream devices such as Mining
/// Proxies and Job Negotiators to handle responses from upstream nodes when
/// attempting to open a Job Negotiation connection.
pub trait JobNegotiationInitiator {
    fn handle_jn_conn_success(conn_success: job_negotiation::SetupConnectionSuccess);
    fn handle_jn_conn_error(conn_error: job_negotiation::SetupConnectionError);
}
