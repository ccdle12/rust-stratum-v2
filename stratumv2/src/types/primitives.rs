use crate::{
    codec::{ByteParser, Deserializable, Serializable},
    error::Result,
};
use std::io;

impl Serializable for bool {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let buffer = if *self { vec![1u8] } else { vec![0u8] };
        writer.write(&buffer)?;
        Ok(buffer.len())
    }
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

// TODO: The specs states that any bits OUTSIDE of the LSB MUST NOT be interpreted
// for the meaning of the bool.
//
// It DOES suggest that the outside bits maybe reused to be interpreted with
// additional meaning in the future. So if we want to be able to allow for this,
// ser/der and initialization of the BOOL type may need to be redesigned.
impl Deserializable for bool {
    fn deserialize(parser: &mut ByteParser) -> Result<bool> {
        let mut buffer: [u8; 1] = [0; 1];
        buffer.clone_from_slice(parser.next_by(1)?);

        Ok(buffer[0] & 1 == 1)
    }
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

#[cfg(test)]
mod tests {
    use crate::codec::{deserialize, serialize};

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
    fn bool_serde() {
        assert_eq!(serialize(&true).unwrap(), vec![1u8]);
        assert_eq!(serialize(&false).unwrap(), vec![0u8]);
        assert_eq!(deserialize::<bool>(&vec![1u8]).unwrap(), true);
        assert_eq!(deserialize::<bool>(&vec![0u8]).unwrap(), false);

        // Deserialize other values that should ONLY interpret the set or
        // unset LSB.
        assert_eq!(deserialize::<bool>(&vec![2u8]).unwrap(), false);
        assert_eq!(deserialize::<bool>(&vec![3u8]).unwrap(), true);
        assert_eq!(deserialize::<bool>(&vec![4u8]).unwrap(), false);
        assert_eq!(deserialize::<bool>(&vec![u8::MAX]).unwrap(), true);
    }
}
