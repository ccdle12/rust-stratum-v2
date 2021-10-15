/// A utility macro to be able to apply all the required traits for variadic
/// number of error code enum variants.
#[macro_export]
macro_rules! impl_error_code_enum {
    ($name:ident, $($variant:path => $str:expr),*) => {
        use std::{convert::TryFrom, fmt, io, str::FromStr};
        use stratumv2_serde::{types::STR0_255, ByteParser, Deserializable, Serializable};

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
                match *self {
                    $($variant => write!(f, $str)),*
                }
            }
        }

        impl FromStr for $name {
            type Err = stratumv2_serde::Error;

            fn from_str(s: &str) -> Result<Self, stratumv2_serde::Error> {
                match s {
                    $($str => Ok($variant)),*,
                    _ => Err(stratumv2_serde::Error::UnknownErrorCode()),
                }
            }
        }

        impl TryFrom<String> for $name {
            type Error = stratumv2_serde::Error;

            fn try_from(s: String) -> Result<Self, stratumv2_serde::Error> {
                Self::from_str(s.as_str())
            }
        }

        impl TryFrom<STR0_255> for $name {
            type Error = stratumv2_serde::Error;

            fn try_from(s: STR0_255) -> Result<Self, stratumv2_serde::Error> {
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
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize, stratumv2_serde::Error> {
                Ok(STR0_255::from(self).serialize(writer)?)
            }
        }

        impl Deserializable for $name {
            fn deserialize(parser: &mut ByteParser) -> Result<$name, stratumv2_serde::Error> {
                let error_code = STR0_255::deserialize(parser)?;
                $name::try_from(error_code)
            }
        }
    };
}
