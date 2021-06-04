use crate::common::SetupConnectionErrorCode;
use crate::impl_setup_connection_error;
use crate::mining::SetupConnectionFlags;

impl_setup_connection_error!(SetupConnectionFlags);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_setup_connection_error_tests;

    impl_setup_connection_error_tests!(SetupConnectionFlags);
}
