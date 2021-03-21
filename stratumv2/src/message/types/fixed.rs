use crate::error::{Error, Result};
use crate::message::parse::{ByteParser, Deserializable, Serializable};
use std::convert::TryFrom;
use std::io;

/// U24 is an unsigned integer type of 24-bits in little endian. This will
/// usually be used to represent the length of a variable-length string or
/// byte-stream.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct U24(pub(crate) u32);

impl U24 {
    pub const MIN: u32 = 0;
    pub const MAX: u32 = 2u32.pow(24) - 1;
    pub const BITS: u32 = 24;

    pub fn new(value: u32) -> Result<U24> {
        use std::convert::TryInto;
        value.try_into()
    }
}

impl PartialEq<u32> for U24 {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<U24> for u32 {
    fn eq(&self, other: &U24) -> bool {
        *self == other.0
    }
}

impl From<U24> for u32 {
    fn from(u: U24) -> Self {
        u.0
    }
}

impl TryFrom<u32> for U24 {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        U24::try_from(value as usize)
    }
}

impl From<U24> for usize {
    fn from(u: U24) -> Self {
        u.0 as usize
    }
}

impl TryFrom<usize> for U24 {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self> {
        if value > usize::pow(2, 24) - 1 {
            return Err(Error::RequirementError(
                "U24 cannot be greater than 2^24-1".into(),
            ));
        }

        Ok(U24(value as u32))
    }
}

impl Deserializable for U24 {
    fn deserialize(parser: &mut ByteParser) -> Result<U24> {
        // Pad the 3-byte slice up to a 4-byte array.
        let mut buffer: [u8; 4] = [0; 4];
        buffer[..3].clone_from_slice(parser.next_by(3)?);

        U24::new(u32::from_le_bytes(buffer))
    }
}

impl Serializable for U24 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&self.0.to_le_bytes()[0..2])?;
        Ok(3)
    }
}

/// U256 is an unsigned integer type of 256-bits in little endian. This will
/// usually be used to represent a raw SHA256 byte output.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct U256(pub(crate) [u8; 32]);

impl Deserializable for U256 {
    fn deserialize(parser: &mut ByteParser) -> Result<U256> {
        let mut buffer: [u8; 32] = [0; 32];
        buffer[..].clone_from_slice(parser.next_by(32)?);
        Ok(U256(buffer))
    }
}

impl Serializable for U256 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        writer.write(&self.0)?;
        Ok(32)
    }
}

impl PartialEq<[u8; 32]> for U256 {
    fn eq(&self, other: &[u8; 32]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<U256> for [u8; 32] {
    fn eq(&self, other: &U256) -> bool {
        *self == other.0
    }
}

impl From<U256> for [u8; 32] {
    fn from(u: U256) -> Self {
        u.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::parse::{deserialize, serialize};

    fn make_encoded_u24(value: u32) -> Vec<u8> {
        value.to_le_bytes()[..3].to_vec()
    }

    #[test]
    fn u24_new() {
        assert!(matches!(U24::new(0), Ok(U24(0))));
        assert!(matches!(U24::new(1), Ok(U24(1))));
        assert!(matches!(U24::new(2u32.pow(24) - 1), Ok(U24(U24::MAX))));
        assert!(matches!(
            U24::new(2u32.pow(24)),
            Err(Error::RequirementError { .. })
        ));
        assert!(matches!(
            U24::new(2u32.pow(24) + 1),
            Err(Error::RequirementError { .. })
        ));
    }

    #[test]
    fn u24_serde_ok() {
        let encoded = make_encoded_u24(5);
        let decoded = U24::new(5).unwrap();

        assert!(matches!(deserialize::<U24>(&encoded), Ok(decoded)));
        assert!(matches!(serialize(&decoded), Ok(encoded)));
    }

    #[test]
    fn u24_deserialize_err() {
        let encoded: [u8; 2] = [0; 2];
        assert!(matches!(
            deserialize::<U24>(&encoded),
            Err(Error::ParseError { .. })
        ));
    }

    #[test]
    fn u24_serialize_err() {
        // Since we are performing bounds-checking in the U24::new factory, we will not be
        // performing any bounds-checking on serialization.
        let encoded = make_encoded_u24(0);
        assert!(matches!(serialize(&U24(2u32.pow(24))), Ok(encoded)));
    }

    fn make_encoded_u256(a: u64, b: u64, c: u64, d: u64) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.extend_from_slice(&a.to_le_bytes());
        buffer.extend_from_slice(&b.to_le_bytes());
        buffer.extend_from_slice(&c.to_le_bytes());
        buffer.extend_from_slice(&d.to_le_bytes());
        return buffer;
    }

    #[test]
    fn u256_serde_ok() {
        let encoded = make_encoded_u256(1, 2, 3, 4);
        let decoded = U256([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 1
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 2
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 3
            0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 4
        ]);
        assert!(matches!(deserialize::<U256>(&encoded), Ok(decoded)));
        assert!(matches!(serialize(&decoded), Ok(encoded)));
    }

    #[test]
    fn u256_deserialize_err() {
        let encoded: [u8; 31] = [0; 31];
        assert!(matches!(
            deserialize::<U256>(&encoded),
            Err(Error::ParseError { .. })
        ));
    }
}
