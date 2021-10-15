use crate::SetupConnectionFlags;
use stratumv2_messages_sdk::impl_setup_connection_error;

// This SetupConnectionError message will contain SetupConnectionFlags specific
// to the mining subprotocol.
impl_setup_connection_error!(SetupConnectionFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use stratumv2_messages_sdk::impl_setup_connection_error_tests;

    impl_setup_connection_error_tests!(SetupConnectionFlags);
}
