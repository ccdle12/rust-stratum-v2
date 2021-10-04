use crate::{
    codec::frame,
    common::{SetupConnection, SetupConnectionErrorCode},
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

/// A default implementation of a message handler for a Pool Server.
struct PoolServerHandler {}

impl<E: Encryptor> ServerMsgHandler<E> for PoolServerHandler {
    fn handle_new_conn(
        &self,
        // TODO: Create the HashMap<Protocol, u32>?
        // TODO: Check the protocol version and if its not in range of the servers
        // version then return an error.
        //
        // server_protocols: HashMap<Protocol, u32>
        server_flags: &mining::SetupConnectionFlags,
        new_conn: SetupConnection,
        peer: &mut Peer<E>,
    ) -> Result<()> {
        match &new_conn {
            SetupConnection::Mining(msg) => {
                // TODO: This is completely irrelevant because all pool servers need to support
                // mining
                // if server_protocols.contains()

                // If a SetupConnection message contains flags that are NOT
                // supported by the server, the server MUST respond to the
                // client with ALL the flags the server does not support.
                if !(msg.flags ^ *server_flags).is_empty() {
                    let all_flags = mining::SetupConnectionFlags::all();
                    let non_supported_flags = all_flags ^ *server_flags;

                    let setup_conn_err = mining::SetupConnectionError::new(
                        non_supported_flags,
                        SetupConnectionErrorCode::UnsupportedFeatureFlags,
                    )?;

                    {
                        let mut msg_buffer = peer.pending_msg_buffer.lock().unwrap();
                        let msg = frame(&setup_conn_err)?;
                        msg_buffer.push(msg);
                    }

                    return Ok(());
                }

                // TODO: Need to check the version of the protocol the server
                // is using, this indicates the server has a config where
                // the server has the version of the protocol its running.
                // let protocol_version = server_protocols.get(Mining).unwrap();
                //
                // if protocol_version < new_conn.min_version && protocol_version > new_conn.max_version {
                //
                // if !(msg.min_version..msg.max_version).contains(&protocol_version) {
                //  ...send a SetupConnectionError with incorrect protocol version.
                // }

                // TODO: This needs to send a sucess
                // Send a SetupConnection.Success using the protocol version of the server.
                peer.setup_conn_msg = Some(new_conn);
            }
            // TODO: Need to check the supported protocols by the server.
            // This indicates the server has a config that contains the
            // protocols it supports.
            // TODO: SetupConnection::JobNegotiation(msg) => {
            //   if !server_protocols.contains(Protocol::JobNegotiation) {
            //     ...send a SetupConnectionError { UnsupportedProtocol }
            //   }
            // }
            _ => (),
        }

        Ok(())
    }
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
