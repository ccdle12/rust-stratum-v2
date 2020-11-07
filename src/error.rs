use std::fmt;
use std::io;

/// Error is the main error type for this library.
#[derive(Debug)]
pub enum Error {
    VersionError(String),
    IOErorr(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::VersionError(ref message) => write!(f, "{}", message),
            Error::IOErorr(ref message) => write!(f, "{}", message),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IOErorr(err)
    }
}

/// Alias Result type for the library.
pub type Result<T> = std::result::Result<T, Error>;
