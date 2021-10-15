pub mod macro_prelude {
    pub use crate::impl_message;
    pub use stratumv2_serde::types::MessageType;
}

#[macro_export]
macro_rules! impl_setup_connection_success {
    ($flags_type:ident) => {
        use stratumv2_messages_sdk::setup_connection_success::macro_prelude::*;

        impl_message!(
            /// TODO: Add a correct doc string and example
            SetupConnectionSuccess,

            /// Version proposed by the connecting node as one of the verions supported
            /// by the upstream node. The version will be used during the lifetime of
            /// the connection.
            used_version u16,

            /// Indicates the optional features the server supports.
            flags $flags_type
        );

        impl SetupConnectionSuccess {
            /// Constructor for the SetupConnectionSuccess message.
            pub fn new(used_version: u16, flags: $flags_type) -> Result<SetupConnectionSuccess, stratumv2_serde::Error> {
                Ok(SetupConnectionSuccess {
                    used_version,
                    flags,
                })
            }
        }
    };
}

pub mod test_macro_prelude {
    pub use crate::impl_message_tests;
}

#[macro_export]
macro_rules! impl_setup_connection_success_tests {
    ($flags_type:ident) => {
        use stratumv2_messages_sdk::setup_connection_success::test_macro_prelude::*;

        fn make_deserialized_setup_connection_success() -> SetupConnectionSuccess {
            SetupConnectionSuccess::new(2, $flags_type::all()).unwrap()
        }

        fn make_serialized_setup_connection_success() -> Vec<u8> {
            let mut serialized = vec![
                0x02, 0x00, // used_version
            ];
            serialized.extend($flags_type::all().bits().to_le_bytes().iter()); // flags
            serialized
        }

        impl_message_tests!(
            SetupConnectionSuccess,
            make_serialized_setup_connection_success,
            make_deserialized_setup_connection_success
        );
    };
}
