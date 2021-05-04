use std::fmt;
use std::io;

/// Error is the main error type for this library.
#[derive(Debug)]
pub enum Error {
    VersionError(String),
    IOError(io::Error),
    Utf8Error(std::str::Utf8Error),
    FromUtf8Error(std::string::FromUtf8Error),
    ProtocolMismatchError(String),
    RequirementError(String),
    DeserializationError(String),
    ParseError(String),
    AuthorityKeyError(ed25519_dalek::ed25519::Error),
    SystemTimeError(std::time::SystemTimeError),
    TryFromSliceError(std::array::TryFromSliceError),
    Unimplemented(),
    UnknownErrorCode(),
    UnknownMessageType(),
    UnknownFlags(),
    UnexpectedMessageType(u16, u8),
    UnexpectedChannelBit(bool),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::VersionError(ref message) => write!(f, "{}", message),
            Error::IOError(ref message) => write!(f, "{}", message),
            Error::Utf8Error(ref message) => write!(f, "{}", message),
            Error::FromUtf8Error(ref message) => write!(f, "{}", message),
            Error::ProtocolMismatchError(ref message) => write!(f, "{}", message),
            Error::RequirementError(ref message) => write!(f, "{}", message),
            Error::DeserializationError(ref message) => write!(f, "{}", message),
            Error::ParseError(ref message) => write!(f, "{}", message),
            Error::AuthorityKeyError(ref message) => write!(f, "{}", message),
            Error::SystemTimeError(ref message) => write!(f, "{}", message),
            Error::TryFromSliceError(ref message) => write!(f, "{}", message),
            Error::Unimplemented() => write!(f, "unimplemented"),
            Error::UnknownErrorCode() => write!(f, "the error code is invalid"),
            Error::UnknownMessageType() => write!(f, "the received message type is unknown"),
            Error::UnknownFlags() => write!(f, "the received flags are unknown"),
            Error::UnexpectedMessageType(ref ext_type, ref msg_type) => {
                write!(
                    f,
                    "parsed message type {}/{} does not match expected message",
                    ext_type, msg_type
                )
            }
            Error::UnexpectedChannelBit(ref channel_bit) => {
                write!(
                    f,
                    "parsed channel bit {} does not match expected message",
                    channel_bit
                )
            }
        }
    }
}

// TODO(chpatton013): Consider replacing this with `thiserror`
/// An internal macro for implementing the From trait for existing Error types
/// into the projects Error type variants.
macro_rules! impl_error_conversions {
    ($($error_type:path => $error_variant:path),*) => {
        $(impl From<$error_type> for Error {
            fn from(err: $error_type) -> Error {
                $error_variant(err)
            }
        })*
    };
}

impl_error_conversions!(
    std::str::Utf8Error => Error::Utf8Error,
    std::string::FromUtf8Error => Error::FromUtf8Error,
    io::Error => Error::IOError,
    ed25519_dalek::ed25519::Error => Error::AuthorityKeyError,
    std::time::SystemTimeError => Error::SystemTimeError,
    std::array::TryFromSliceError => Error::TryFromSliceError
);

/// Alias Result type for the library.
pub type Result<T> = std::result::Result<T, Error>;
