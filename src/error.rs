use std::fmt;
use std::io;

/// Error is the main error type for this library.
#[derive(Debug)]
pub enum Error {
    VersionError(String),
    IOErorr(io::Error),
    Utf8Error(std::str::Utf8Error),
    ProtocolMismatchError(String),
    RequirementError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::VersionError(ref message) => write!(f, "{}", message),
            Error::IOErorr(ref message) => write!(f, "{}", message),
            Error::Utf8Error(ref message) => write!(f, "{}", message),
            Error::ProtocolMismatchError(ref message) => write!(f, "{}", message),
            Error::RequirementError(ref message) => write!(f, "{}", message),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOErorr(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::Utf8Error(err)
    }
}

/// Alias Result type for the library.
pub type Result<T> = std::result::Result<T, Error>;
