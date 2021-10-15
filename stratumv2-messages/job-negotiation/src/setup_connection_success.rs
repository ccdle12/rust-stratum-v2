use stratumv2_messages_sdk::impl_setup_connection_success;
use stratumv2_serde::impl_bitflags_serde;

bitflags!(
    /// Feature flags for the SetupConnectionSuccess message from the Server to
    /// the Client for the JobNegotiation Protocol.
    pub struct SetupConnectionSuccessFlags: u32 {
        const NONE = 0;
    }
);

impl_bitflags_serde!(SetupConnectionSuccessFlags);

// SetupConnectionSuccess is an implementation of the SetupConnectionSuccess
// message specific to the job negotiation subprotocol and will contain the
// job negotiation SetupConnectionSuccessFlags.
impl_setup_connection_success!(SetupConnectionSuccessFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use stratumv2_messages_sdk::impl_setup_connection_success_tests;

    impl_setup_connection_success_tests!(SetupConnectionSuccessFlags);
}
