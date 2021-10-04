use crate::impl_error_code_enum;

/// Contains the error codes for the [SetupConnectionError](struct.SetupConnectionError.html) message.
/// Each error code has a default STR0_255 message.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SetupConnectionErrorCode {
    /// Indicates the server has received a feature flag from a client that
    /// the server does not support.
    UnsupportedFeatureFlags,

    /// Indicates the server has received a connection request using a protcol
    /// the server does not support.
    UnsupportedProtocol,

    /// Indicates the server has received a protocol version that is not currently
    /// supported by the server.
    ProtocolVersionMismatch,
}

impl_error_code_enum!(
    SetupConnectionErrorCode,
    SetupConnectionErrorCode::UnsupportedFeatureFlags => "unsupported-feature-flags",
    SetupConnectionErrorCode::UnsupportedProtocol => "unsupported-protocol",
    SetupConnectionErrorCode::ProtocolVersionMismatch => "protocol-version-mismatch"
);
