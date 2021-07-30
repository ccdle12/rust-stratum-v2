use stratumv2::{
    codec::frame,
    common::{SetupConnection, SetupConnectionErrorCode},
    error::Result,
    mining,
    mining::SetupConnectionError,
    network::{ChannelManager, Encryptor, Peer, ServerConfig, ServerMsgHandler},
    types::MessageType,
};

// TODO: Move to functional_tests_util.rs
struct MockEncryptor {}

impl Encryptor for MockEncryptor {
    fn is_handshake_complete(&self) -> bool {
        true
    }

    fn recv_handshake(&mut self, bytes: &mut [u8]) -> Result<Vec<u8>> {
        Ok(vec![0])
    }

    fn init_handshake(&mut self) -> Result<Vec<u8>> {
        Ok(vec![0])
    }

    fn encrypt_message(bytes: &[u8]) -> Vec<u8> {
        vec![0]
    }

    fn decrypt_message(bytes: &[u8]) -> Vec<u8> {
        vec![0]
    }
}

// TODO: Move to functional_tests_util.rs or define a default server message handler.
struct MsgHandler {}

impl<E: Encryptor> ServerMsgHandler<E> for MsgHandler {
    fn handle_new_conn(
        &self,
        server_flags: &stratumv2::mining::SetupConnectionFlags,
        new_conn: SetupConnection,
        peer: &mut Peer<E>,
    ) -> Result<()> {
        match &new_conn {
            SetupConnection::Mining(m) => {
                let msg_contains_unsupported_flags =
                    |server_flags: &stratumv2::mining::SetupConnectionFlags,
                     msg: &stratumv2::mining::SetupConnection| {
                        !(*server_flags ^ msg.flags).is_empty()
                    };

                // If after XOR the server_config with the message is greater
                // than 0, then we have unsupported flags.
                if msg_contains_unsupported_flags(&server_flags, &m) {
                    let all_flags = stratumv2::mining::SetupConnectionFlags::all();
                    let non_supported_flags = all_flags ^ *server_flags;

                    let setup_conn_err = SetupConnectionError::new(
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

                // TODO: Need to check the protocol version supported
                // by the server.
                //
                // TODO: This needs to send a sucess
                //
                // TODO: This should be the last step.
                peer.setup_conn_msg = Some(new_conn);
            }
            _ => (),
        }

        Ok(())
    }
}

#[test]
fn setup_mining_conn() {
    // 1. Create a client peer
    let mut client = Peer::new(MockEncryptor {});

    let server_flags = mining::SetupConnectionFlags::REQUIRES_STANDARD_JOBS;

    // 3. Create a server config
    let server_config = ServerConfig::new(server_flags);
    let server_msg_handler = MsgHandler {};

    // 4. Create a SetupConnection Message
    let new_conn = SetupConnection::new_mining(
        2,
        2,
        mining::SetupConnectionFlags::all(),
        "0.0.0.0",
        8545,
        "Bitmain",
        "S9i 13.5",
        "braiins-os-2018-09-22-1-hash",
        "some-device-uuid",
    )
    .unwrap();

    // 5. Client sends a SetupConnection Message by calling the ServerMsgHandler.handle()
    server_msg_handler.handle_new_conn(&server_config.mining_flags, new_conn, &mut client);

    // 6. The server processes it and puts a response on its buffer for the peer.
    let expected_msg = client.get_pending_msgs()[0].message_type;

    // 7. Read the buffer and assert the message type.
    assert_eq!(expected_msg, MessageType::SetupConnectionError);
}

// TODO:
// 1. Implement a message handler for the server.
// 2. Pass in the SetupConnection and Peer and ServerFlag.
// 3. Start moving the separate pieces into larger pieces to abstract away the interface. The
//    functional test should just be testing the overall interface.
