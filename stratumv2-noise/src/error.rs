use thiserror::Error;

/// The main error type for this library.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Base58Error(#[from] bitcoin::util::base58::Error),

    #[error("`{0}`")]
    VersionError(String),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("`{0}`")]
    ProtocolMismatchError(String),

    #[error("`{0}`")]
    RequirementError(String),

    #[error("`{0}`")]
    DeserializationError(String),

    #[error("`{0}`")]
    ParseError(String),

    #[error(transparent)]
    AuthorityKeyError(#[from] ed25519_dalek::ed25519::Error),

    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),

    #[error(transparent)]
    TryFromSliceError(#[from] std::array::TryFromSliceError),

    #[error("Unimplemented")]
    Unimplemented(),

    #[error("The error code is invalid")]
    UnknownErrorCode(),

    #[error("the received message type is unknown")]
    UnknownMessageType(),

    #[error("the received flags are unknown")]
    UnknownFlags(),

    #[error("parsed message type `{0}/{1}` does not match expected message")]
    UnexpectedMessageType(u16, u8),

    #[error("parsed channel bit `{0}` does not match expected message")]
    UnexpectedChannelBit(bool),

    #[error(transparent)]
    NoiseError(#[from] noiseexplorer_nx::error::NoiseError),

    // TODO:
    #[error(transparent)]
    StratumV2SerdeError(#[from] stratumv2_serde::Error),
}

/// Alias Result type for the library.
pub type Result<T> = std::result::Result<T, Error>;
