use crate::common::types::U256;

/// OpenStandardMiningChannel is a message sent by the client to the server
/// after a [SetupConnection.Success](struct.SetupConnectionSuccess.html) is
/// sent by the server. This message is used to request opening a standard
/// channel to the upstream server. A standard mining channel indicates `header-only`
/// mining.
pub struct OpenStandardMiningChannel {
    pub request_id: u32,
    pub user_identity: String,
    pub nominal_hash_rate: f32,
    pub max_target: U256,
}

impl OpenStandardMiningChannel {
    /// Constructor for the OpenStandardMiningChannel message.
    fn new(
        request_id: u32,
        user_identity: String,
        nominal_hash_rate: f32,
        max_target: U256,
    ) -> OpenStandardMiningChannel {
        OpenStandardMiningChannel {
            request_id,
            user_identity,
            nominal_hash_rate,
            max_target,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_open_standard_mining_channel_0() {
        // TODO: Substitute the nominal hash rate with something more accurate.
        let target = [0u8; 32];

        let message =
            OpenStandardMiningChannel::new(1, "braiinstest.worker1".to_string(), 12.3, target);

        assert_eq!(message.request_id, 1);
        assert_eq!(message.user_identity, "braiinstest.worker1");
        assert_eq!(message.nominal_hash_rate, 12.3);
        assert_eq!(message.max_target.len(), 32);
    }
}
