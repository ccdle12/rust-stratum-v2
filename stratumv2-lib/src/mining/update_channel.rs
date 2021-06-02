use crate::{error::Result, impl_message, types::U256};

impl_message!(
    /// UpdateChannel is sent from the Client to a Server. This message is used by
    /// the Client to notify the server about specific changes to a channel.
    UpdateChannel,

    /// The unique identifier of the channel.
    channel_id u32,

    /// The expected [h/s] (hash rate/per second) of the device or the
    /// cumulative rate on the channel if multiple devices are connected
    /// downstream. Proxies MUST send 0.0f when there are no mining devices
    /// connected yet.
    nominal_hash_rate f32,

    /// The Max Target that can be acceptd by the connected device or
    /// multiple devices downstream. In this case, if the max_target of
    /// the channel is smaller than the current max target, the Server MUST
    /// respond with a SetTarget message.
    max_target U256
);

impl UpdateChannel {
    pub fn new(channel_id: u32, nominal_hash_rate: f32, max_target: U256) -> Result<UpdateChannel> {
        Ok(UpdateChannel {
            channel_id,
            nominal_hash_rate,
            max_target,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::impl_message_tests;

    fn make_deserialized_update_channel() -> UpdateChannel {
        UpdateChannel::new(1, 12.3, U256([0; 32])).unwrap()
    }

    fn make_serialized_update_channel() -> Vec<u8> {
        return vec![
            0x01, 0x00, 0x00, 0x00, // channel_id
            0xcd, 0xcc, 0x44, 0x41, // nominal_hash_rate
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, // max_target
        ];
    }

    impl_message_tests!(
        UpdateChannel,
        make_serialized_update_channel,
        make_deserialized_update_channel
    );
}
