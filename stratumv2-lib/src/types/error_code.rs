pub mod macro_prelude {
    pub use crate::error::{Error, Result};
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::STR0_255;
    pub use std::convert::TryFrom;
    pub use std::fmt;
    pub use std::io;
    pub use std::str::FromStr;
}

/// Implemenation of all the common traits for ErrorCode enums.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_error_code_enum {
    ($name:ident, $($variant:path => $str:expr),*) => {
        use crate::types::error_code::macro_prelude::*;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $($variant => write!(f, $str)),*
                }
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $($str => Ok($variant)),*,
                    _ => Err(Error::UnknownErrorCode()),
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = Error;

            fn try_from(s: String) -> Result<Self> {
                Self::from_str(s.as_str())
            }
        }

        impl TryFrom<STR0_255> for $name {
            type Error = Error;

            fn try_from(s: STR0_255) -> Result<Self> {
                Self::try_from(s.data)
            }
        }

        impl From<&$name> for String {
            fn from(error_code: &$name) -> Self {
                match error_code {
                    $($variant => $str.into()),*
                }
            }
        }

        impl From<&$name> for STR0_255 {
            fn from(error_code: &$name) -> Self {
                let data: String = error_code.into();
                STR0_255{
                    length: data.len() as u8,
                    data: data,
                }
            }
        }

        impl Serializable for $name {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                Ok(STR0_255::from(self).serialize(writer)?)
            }
        }

        impl Deserializable for $name {
            fn deserialize(parser: &mut ByteParser) -> Result<$name> {
                let error_code = STR0_255::deserialize(parser)?;

                $name::try_from(error_code)
            }
        }
    };
}
