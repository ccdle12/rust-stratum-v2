pub mod macro_prelude {
    pub use crate::common::SetupConnectionErrorCode;
    pub use crate::error::{Error, Result};
    pub use crate::impl_message;
    pub use crate::types::{MessageType, STR0_255};
    pub use std::io;
}

/// Implementation of the SetupConnectionError message for each sub protocol.
#[macro_export]
macro_rules! impl_setup_connection_error {
    ($flags_type:ident) => {
        use crate::macro_message::setup_connection_error::macro_prelude::*;

        impl_message!(
            /// SetupConnectionError is one of the required responses from a Server
            /// to a Client when a new connection has failed. The server is required
            /// to send this message with an error code before closing the connection.
            ///
            /// If the error is a variant of [UnsupportedFeatureFlags](enum.SetupConnectionErrorCode.html),
            /// the server MUST respond with all the feature flags that it does NOT support.
            ///
            /// If the flag is 0, then the error is some condition aside from unsupported
            /// flags.
            ///
            /// # Examples
            ///
            /// ```rust
            /// use stratumv2_lib::mining;
            /// use stratumv2_lib::common::SetupConnectionErrorCode;
            ///
            /// let conn_error = mining::SetupConnectionError::new(
            ///     mining::SetupConnectionFlags::REQUIRES_VERSION_ROLLING,
            ///     SetupConnectionErrorCode::UnsupportedFeatureFlags
            /// );
            ///
            /// assert!(conn_error.is_ok());
            /// assert_eq!(
            ///     conn_error.unwrap().error_code,
            ///     SetupConnectionErrorCode::UnsupportedFeatureFlags
            /// );
            SetupConnectionError,
            MessageType::SetupConnectionError,
            /// Indicates all the flags that the server does NOT support.
            pub flags $flags_type,
            /// Error code is a predefined STR0_255 error code.
            pub error_code SetupConnectionErrorCode
        );

        impl SetupConnectionError {
            pub fn new(
                flags: $flags_type,
                error_code: SetupConnectionErrorCode,
            ) -> Result<SetupConnectionError> {
                if flags.is_empty()
                    && error_code == SetupConnectionErrorCode::UnsupportedFeatureFlags
                {
                    return Err(Error::RequirementError(
                        "a full set of unsupported flags MUST be returned to the client".into(),
                    ));
                }

                Ok(SetupConnectionError { flags, error_code })
            }
        }
    };
}
