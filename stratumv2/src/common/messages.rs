use std::fmt;

/// Contains the error codes for the [SetupConnectionError](struct.SetupConnectionError.html) message.
/// Each error code has a default STR0_255 message.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SetupConnectionErrorCodes {
    /// Indicates the server has received a feature flag from a client that
    /// the server does not support.
    UnsupportedFeatureFlags,

    /// Indicates the server has received a connection request using a protcol
    /// the server does not support.
    UnsupportedProtocol,

    // TODO: What is the difference between protocol version mismatch
    // and unsupported protocol?
    ProtocolVersionMismatch,

    // TODO: Review this, I don't like it
    UnknownFlag,
}

impl fmt::Display for SetupConnectionErrorCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetupConnectionErrorCodes::UnsupportedFeatureFlags => {
                write!(f, "unsupported-feature-flags")
            }
            SetupConnectionErrorCodes::UnsupportedProtocol => write!(f, "unsupported-protocol"),
            SetupConnectionErrorCodes::ProtocolVersionMismatch => {
                write!(f, "protocol-version-mismatch")
            }

            // TODO: Review this, I don't like it
            SetupConnectionErrorCodes::UnknownFlag => write!(f, "unknown flag"),
        }
    }
}

impl From<&str> for SetupConnectionErrorCodes {
    fn from(error_code: &str) -> Self {
        match error_code {
            "unsupported-feature-flags" => SetupConnectionErrorCodes::UnsupportedFeatureFlags,
            "unsupported-protocol" => SetupConnectionErrorCodes::UnsupportedProtocol,
            "protocol-version-mismatch" => SetupConnectionErrorCodes::ProtocolVersionMismatch,

            // TODO: Review this, I don't like it
            _ => SetupConnectionErrorCodes::UnknownFlag,
        }
    }
}
