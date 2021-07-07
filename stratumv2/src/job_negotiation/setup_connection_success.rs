use crate::{impl_bitflags_serde, impl_setup_connection_success};

bitflags!(
    /// Feature flags for the SetupConnectionSuccess message from the Server to
    /// the Client for the JobNegotiation Protocol.
    pub struct SetupConnectionSuccessFlags: u32 {
        const NONE = 0;
    }
);

impl_bitflags_serde!(SetupConnectionSuccessFlags);
impl_setup_connection_success!(SetupConnectionSuccessFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_setup_connection_success_tests;

    impl_setup_connection_success_tests!(SetupConnectionSuccessFlags);
}
