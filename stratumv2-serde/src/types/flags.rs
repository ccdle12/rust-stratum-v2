pub mod macro_prelude {
    pub use crate::Error;
    pub use crate::{ByteParser, Deserializable, Serializable};
}

/// Implemenation of all the ser/de traits for bitflags.
#[macro_export]
macro_rules! impl_bitflags_serde {
    ($name:ident) => {
        impl_bitflags_serde!($name, u32);
    };

    ($name:ident, $underlying:ident) => {
        use stratumv2_serde::types::flags::macro_prelude::*;

        impl Serializable for $name {
            fn serialize<W: io::Write>(
                &self,
                writer: &mut W,
            ) -> Result<usize, stratumv2_serde::Error> {
                Ok(self.bits().serialize(writer)?)
            }
        }

        impl Deserializable for $name {
            fn deserialize(parser: &mut ByteParser) -> Result<$name, stratumv2_serde::Error> {
                $name::from_bits($underlying::deserialize(parser)?).ok_or(Error::UnknownFlags())
            }
        }
    };
}
