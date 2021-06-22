use crate::{common::SetupConnection, job_negotiation, mining};

/// The main trait for any networked message handlers. The behaviour of the
/// MessageHandlers `handle_message()` function should receives bytes,
/// deframe the bytes and delegate the message handling to specific handlers
/// according to the received message type.
pub trait MessageHandler {
    fn handle_message(bytes: &[u8]);
}

/// A trait that should be applied to upstream devices such as a Mining Pool Server
/// that can handle [SetupConnection Messages](../common/setup_connection/enum.SetupConnection.html).
pub trait NewConnReceiver {
    fn handle_new_conn(new_conn: SetupConnection);
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
