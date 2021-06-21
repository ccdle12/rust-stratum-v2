use crate::error::Result;
use crate::impl_message;
use crate::types::{MessageType, STR0_255, U256};

impl_message!(
    /// A message sent by the Client to the Server after a [SetupConnection.Success](struct.SetupConnectionSuccess.html)
    /// is sent from the Server. This message is used to request opening a standard
    /// channel to the upstream server. A standard mining channel indicates `header-only`
    /// mining.
    OpenStandardMiningChannel,

    /// A Client-specified unique identifier across all client connections.
    /// The request_id is not interpreted by the Server.
    request_id u32,

    /// A sequence of bytes that identifies the node to the Server, e.g.
    /// "braiintest.worker1".
    user_identity STR0_255,

    /// The expected [h/s] (hash rate/per second) of the
    /// device or the cumulative on the channel if multiple devices are connected
    /// downstream. Proxies MUST send 0.0f when there are no mining devices
    /// connected yet.
    nominal_hash_rate f32,

    /// The Maximum Target that can be acceptd by the connected device or
    /// multiple devices downstream. The Server MUST accept the maximum
    /// target or respond by sending a
    /// [OpenStandardMiningChannel.Error](struct.OpenStandardMiningChannelError.html)
    /// or [OpenExtendedMiningChannel.Error](struct.OpenExtendedMiningChannelError.html)
    max_target U256
);

impl OpenStandardMiningChannel {
    pub fn new<T: Into<String>, U: Into<U256>>(
        request_id: u32,
        user_identity: T,
        nominal_hash_rate: f32,
        max_target: U,
    ) -> Result<OpenStandardMiningChannel> {
        Ok(OpenStandardMiningChannel {
            request_id,
            user_identity: STR0_255::new(user_identity)?,
            nominal_hash_rate,
            max_target: max_target.into(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_message_tests;

    fn make_deserialized_open_standard_mining_channel() -> OpenStandardMiningChannel {
        OpenStandardMiningChannel::new(1u32, "user id", 3.0f32, [4u8; 32]).unwrap()
    }

    fn make_serialized_open_standard_mining_channel() -> Vec<u8> {
        return vec![
            0x01, 0x00, 0x00, 0x00, // request_id,
            0x07, 0x75, 0x73, 0x65, 0x72, 0x20, 0x69, 0x64, // user_identity
            0x00, 0x00, 0x40, 0x40, // nominal_hash_rate
            0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
            0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
            0x04, 0x04, 0x04, 0x04, // max_target
        ];
    }

    impl_message_tests!(
        OpenStandardMiningChannel,
        make_serialized_open_standard_mining_channel,
        make_deserialized_open_standard_mining_channel
    );
}
