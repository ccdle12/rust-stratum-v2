use std::fmt;

/// Error is the main error type for this library.
#[derive(Debug)]
pub enum Error {
    VersionError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::VersionError(ref message) => write!(f, "{}", message),
        }
    }
}

/// Alias Result type for the library.
pub type Result<T> = std::result::Result<T, Error>;
