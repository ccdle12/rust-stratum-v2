use std::convert::From;
use std::fmt;
use std::io;
use stratumv2_lib::bitcoin;

/// Error is the main error type for this library.
#[derive(Debug)]
pub enum Error {
    Base58Error(bitcoin::util::base58::Error),
    ParseError(String),
    IOError(io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Base58Error(ref message) => write!(f, "{}", message),
            Error::IOError(ref message) => write!(f, "{}", message),
            Error::ParseError(ref message) => write!(f, "{}", message),
            Error::TryFromSliceError(ref message) => write!(f, "{}", message),
        }
    }
}

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
    bitcoin::util::base58::Error => Error::Base58Error,
    io::Error => Error::IOError,
    std::array::TryFromSliceError => Error::TryFromSliceError
);

pub type Result<T> = std::result::Result<T, Error>;
