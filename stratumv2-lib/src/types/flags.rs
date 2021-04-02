pub mod macro_prelude {
    pub use crate::error::{Error, Result};
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
}

/// Implemenation of all the ser/de traits for bitflags.
#[macro_export]
macro_rules! impl_bitflags_serde {
    ($name:ident) => {
        impl_bitflags_serde!($name, u32);
    };

    ($name:ident, $underlying:ident) => {
        use crate::types::flags::macro_prelude::*;

        impl Serializable for $name {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let length = self.bits().serialize(writer)?;

                Ok(length)
            }
        }

        impl Deserializable for $name {
            fn deserialize(parser: &mut ByteParser) -> Result<$name> {
                $name::from_bits($underlying::deserialize(parser)?).ok_or(Error::UnknownFlags())
            }
        }
    };
}
