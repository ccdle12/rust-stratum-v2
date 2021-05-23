pub mod macro_prelude {
    pub use crate::common::SetupConnectionErrorCode;
    pub use crate::error::{Error, Result};
    pub use crate::impl_message;
    pub use crate::types::{MessageType, STR0_255};
    pub use std::io;
}

/// Implementation of the SetupConnectionError message for each sub protocol.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_setup_connection_error {
    ($flags_type:ident) => {
        use crate::macro_message::setup_connection_error::macro_prelude::*;

        impl_message!(
            /// One of the required responses from a Server to a Client when a
            /// new connection has failed. The server is required to send this
            /// message with an error code before closing the connection.
            ///
            /// If the error is a variant of [UnsupportedFeatureFlags](../common/enum.SetupConnectionErrorCode.html),
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

            /// Indicates all the flags that the server does NOT support.
            flags $flags_type,

            /// Error code is a predefined STR0_255 error code.
            error_code SetupConnectionErrorCode
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

#[cfg(test)]
pub mod test_macro_prelude {
    pub use crate::impl_message_tests;
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! impl_setup_connection_error_tests {
    ($flags_type:ident) => {
        use crate::macro_message::setup_connection_error::test_macro_prelude::*;

        fn make_deserialized_setup_connection_error() -> SetupConnectionError {
            SetupConnectionError::new(
                $flags_type::all(),
                SetupConnectionErrorCode::UnsupportedFeatureFlags,
            )
            .unwrap()
        }

        fn make_serialized_setup_connection_error() -> Vec<u8> {
            let mut serialized = vec![];
            serialized.extend($flags_type::all().bits().to_le_bytes().iter()); // flags
            serialized.extend(vec![
                // error_code
                0x19, // length (25)
                0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, // "unsuppor"
                0x74, 0x65, 0x64, 0x2d, 0x66, 0x65, 0x61, 0x74, // "ted-feat"
                0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, // "ure-flag"
                0x73, // "s"
            ]);

            serialized
        }

        impl_message_tests!(
            SetupConnectionError,
            make_serialized_setup_connection_error,
            make_deserialized_setup_connection_error
        );

        #[test]
        fn empty_feature_flags_error() {
            assert!(matches!(
                SetupConnectionError::new(
                    $flags_type::empty(),
                    SetupConnectionErrorCode::UnsupportedFeatureFlags
                ),
                Err(Error::RequirementError { .. })
            ));
        }
    };
}
