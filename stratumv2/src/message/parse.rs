use crate::error::{Error, Result};
use std::io;

/// ByteParser is a custom iterator-like struct. It's used to extract segments
/// from a slice using by providing an offset to return the bytes from start
/// to step.
pub struct ByteParser<'a> {
    bytes: &'a [u8],
    start: usize,
}

impl<'a> ByteParser<'a> {
    pub(crate) fn new(bytes: &'a [u8], start: usize) -> ByteParser {
        ByteParser { bytes, start }
    }

    pub(crate) fn next_by(&mut self, step: usize) -> Result<&'a [u8]> {
        let offset = self.start + step;

        let b = self.bytes.get(self.start..offset);
        if b.is_none() {
            return Err(Error::ParseError("out of bounds error".into()));
        }

        self.start = offset;
        Ok(b.unwrap())
    }
}

/// Trait for deserializing bytes to most Stratum V2 messages.
pub trait Deserializable {
    fn deserialize(parser: &mut ByteParser) -> Result<Self>
    where
        Self: std::marker::Sized;
}

impl Deserializable for u8 {
    fn deserialize(parser: &mut ByteParser) -> Result<u8> {
        let mut buffer: [u8; 1] = [0; 1];
        buffer.clone_from_slice(parser.next_by(1)?);

        Ok(u8::from_le_bytes(buffer))
    }
}

impl Deserializable for u16 {
    fn deserialize(parser: &mut ByteParser) -> Result<u16> {
        let mut buffer: [u8; 2] = [0; 2];
        buffer.clone_from_slice(parser.next_by(2)?);

        Ok(u16::from_le_bytes(buffer))
    }
}

impl Deserializable for u32 {
    fn deserialize(parser: &mut ByteParser) -> Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        buffer.clone_from_slice(parser.next_by(4)?);

        Ok(u32::from_le_bytes(buffer))
    }
}

impl Deserializable for f32 {
    fn deserialize(parser: &mut ByteParser) -> Result<f32> {
        let mut buffer: [u8; 4] = [0; 4];
        buffer.clone_from_slice(parser.next_by(4)?);

        Ok(f32::from_le_bytes(buffer))
    }
}

/// Helper utility function to deserialize a byte-stream into a type that
/// implements the Serializable trait and returns the deserialized result.
pub fn deserialize<T: Deserializable>(bytes: &[u8]) -> Result<T> {
    let mut parser = ByteParser::new(bytes, 0);
    T::deserialize(&mut parser)
}

/// Trait for encoding and serializing messages and objects according to the
/// Stratum V2 protocol.
pub trait Serializable {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize>;
}

impl Serializable for u8 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)?;
        Ok(buffer.len())
    }
}

impl Serializable for u16 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)?;
        Ok(buffer.len())
    }
}

impl Serializable for u32 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)?;
        Ok(buffer.len())
    }
}

impl Serializable for f32 {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = self.to_le_bytes();
        writer.write(&buffer)?;
        Ok(buffer.len())
    }
}

/// Helper utility function to serialize a type that implements the Serializable
/// trait and returns the serialized result.
pub fn serialize<T: Serializable>(val: &T) -> Result<Vec<u8>> {
    let mut buffer = vec![];
    val.serialize(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_serde() {
        assert_eq!(serialize(&0b10000001u8).unwrap(), vec![0b10000001u8]);
        assert_eq!(deserialize::<u8>(&[0b10000001u8]).unwrap(), 0b10000001u8);
    }

    #[test]
    fn u16_serde() {
        assert_eq!(
            serialize(&0b01000010_10000001u16).unwrap(),
            vec![0b10000001u8, 0b01000010u8]
        );
        assert_eq!(
            deserialize::<u16>(&[0b10000001u8, 0b01000010u8]).unwrap(),
            0b01000010_10000001u16
        );
    }

    #[test]
    fn u32_serde() {
        assert_eq!(
            serialize(&0b00011000_00100100_01000010_10000001u32).unwrap(),
            vec![0b10000001u8, 0b01000010u8, 0b00100100u8, 0b00011000u8]
        );
        assert_eq!(
            deserialize::<u32>(&[0b10000001u8, 0b01000010u8, 0b00100100u8, 0b00011000u8]).unwrap(),
            0b00011000_00100100_01000010_10000001u32
        );
    }

    #[test]
    fn f32_serde() {
        // Binary representation of PI in 32-bit floating-point:
        //     0 10000000 10010010000111111011011
        assert_eq!(
            serialize(&3.14159274101257324f32).unwrap(),
            vec![0b11011011, 0b00001111, 0b01001001u8, 0b01000000u8]
        );
        assert_eq!(
            deserialize::<f32>(&[0b11011011, 0b00001111, 0b01001001u8, 0b01000000u8]).unwrap(),
            3.14159274101257324f32
        );
    }

    #[test]
    fn parser_next() {
        let bytes: [u8; 4] = [0, 1, 2, 3];

        let mut parser = ByteParser::new(&bytes, 1);
        assert!(matches!(parser.next_by(2), Ok(&[1, 2])));
        assert!(matches!(parser.next_by(2), Err(Error::ParseError { .. })));
        assert!(matches!(parser.next_by(1), Ok(&[3])));
        assert!(matches!(parser.next_by(1), Err(Error::ParseError { .. })));
    }
}
