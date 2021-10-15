pub mod setup_connection_error_macro_prelude {
    pub use crate::{impl_message, setup_connection_error_code::SetupConnectionErrorCode};
    pub use std::io;
    pub use stratumv2_serde::types::MessageType;
}

/// A macro to implement a SetupConnectionError message for each sub-protocol.
#[macro_export]
macro_rules! impl_setup_connection_error {
    ($flags_type:ident) => {
        use stratumv2_messages_sdk::setup_connection_error::setup_connection_error_macro_prelude::*;

        impl_message!(
            /// TODO: TMP - Add the correct docstring and comment
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
            ) -> Result<SetupConnectionError, stratumv2_serde::Error> {
                if flags.is_empty()
                    && error_code == SetupConnectionErrorCode::UnsupportedFeatureFlags
                {
                    return Err(stratumv2_serde::Error::RequirementError(
                        "a full set of unsupported flags MUST be returned to the client".into(),
                    ));
                }

                Ok(SetupConnectionError { flags, error_code })
            }
        }
    };
}

pub mod setup_connection_test_macro_prelude {
    pub use crate::impl_message_tests;
}

#[macro_export]
macro_rules! impl_setup_connection_error_tests {
    ($flags_type:ident) => {
        use stratumv2_messages_sdk::setup_connection_error::setup_connection_test_macro_prelude::*;

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
                0x19, 0x75, 0x6e, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x2d, 0x66,
                0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x2d, 0x66, 0x6c, 0x61, 0x67, 0x73,
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
                Err(stratumv2_serde::Error::RequirementError { .. })
            ));
        }
    };
}
