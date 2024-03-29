pub mod macro_prelude {
    pub use crate::codec::{ByteParser, Deserializable, Serializable};
    pub use crate::error::{Error, Result};
}

/// Implemenation of all the ser/de traits for bitflags.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_bitflags_serde {
    ($name:ident) => {
        impl_bitflags_serde!($name, u32);
    };

    ($name:ident, $underlying:ident) => {
        use crate::types::flags::macro_prelude::*;

        impl Serializable for $name {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                Ok(self.bits().serialize(writer)?)
            }
        }

        impl Deserializable for $name {
            fn deserialize(parser: &mut ByteParser) -> Result<$name> {
                $name::from_bits($underlying::deserialize(parser)?).ok_or(Error::UnknownFlags())
            }
        }
    };
}
