use crate::error::Error::RequirementError;
use crate::error::Result;

/// U256 is an unsigned integer type of 256-bits in little endian. This will
/// usually be used to represent a raw SHA256 byte output.
pub(crate) type U256 = [u8; 32];

// Implement STR0 types.
impl_sized_STR0!(STR0_255, 255);
impl_sized_STR0!(STR0_32, 32);

// TODO: TEMP Create a macro to create B0_32, B0_16M?
// TODO AND NOTE: The specification does contain an official implemenation for
// B0_32. I'm making an assumption that the serialized form will be:
// <1-byte length L (u8) + variable length bytes>
// This should be reviewed.
/// B0_32 is a type representing a vector of bytes with a maximum size of 32 bytes.
/// Serialization is assumed with the following structure:
/// <1-byte length L (u8) + variable length bytes>
#[derive(Debug, Clone)]
pub struct B0_32(Vec<u8>);

impl B0_32 {
    const MAX_SIZE: usize = 32;

    pub fn new<T: Into<Vec<u8>>>(value: T) -> Result<B0_32> {
        let input = value.into();
        if input.len() > Self::MAX_SIZE {
            return Err(RequirementError(
                "length of bytes cannot be greater than 32".into(),
            ));
        }

        Ok(B0_32(input))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        serialize_slices!(&[self.0.len() as u8], &self.0)
    }
}

/// PartialEq implementation allowing direct comparison between B0_32 and Vec<u8>.
impl PartialEq<Vec<u8>> for B0_32 {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.0 == *other
    }
}

/// MessageTypes contain all the variations for the byte representation of
/// messages used in message frames.
// TODO: Create a macro maybe for just conversions to keep it more readable.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MessageTypes {
    SetupConnection,
    SetupConnectionSuccess,
    SetupConnectionError,
    OpenStandardMiningChannel,
    OpenStandardMiningChannelSuccess,
    OpenStandardMiningChannelError,
    OpenExtendedMiningChannelError,
    Unknown,
}

// TODO: A macro that will do conversions both ways.
impl From<MessageTypes> for u8 {
    fn from(m: MessageTypes) -> Self {
        match m {
            MessageTypes::SetupConnection => 0x00,
            MessageTypes::SetupConnectionSuccess => 0x01,
            MessageTypes::SetupConnectionError => 0x03,
            MessageTypes::OpenStandardMiningChannel => 0x10,
            MessageTypes::OpenStandardMiningChannelSuccess => 0x11,
            MessageTypes::OpenStandardMiningChannelError => 0x12,
            MessageTypes::OpenExtendedMiningChannelError => 0x15,
            // TODO: THIS IS NOT SPECIFIED IN THE PROTOCOL.
            MessageTypes::Unknown => 0xFF,
        }
    }
}

impl From<u8> for MessageTypes {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => MessageTypes::SetupConnection,
            0x01 => MessageTypes::SetupConnectionSuccess,
            0x03 => MessageTypes::SetupConnectionError,
            0x10 => MessageTypes::OpenStandardMiningChannel,
            0x11 => MessageTypes::OpenStandardMiningChannelSuccess,
            0x15 => MessageTypes::OpenExtendedMiningChannelError,
            // TODO: THIS IS NOT SPECIFIED IN THE PROTOCOL.
            _ => MessageTypes::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str0_255_init() {
        assert!(STR0_255::new("hello").is_ok());
    }

    #[test]
    fn str0_255_into_string() {
        let input = "hello";
        let str_255 = STR0_255::new(input);
        let result: String = str_255.unwrap().into();

        assert_eq!(result, input);
    }

    #[test]
    fn str0_255_to_bytes() {
        let expected = vec![0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let result: Vec<u8> = STR0_255::new("hello").unwrap().as_bytes();

        assert_eq!(result, expected);
    }

    #[test]
    fn str0_255_size_limit() {
        let mut input = String::with_capacity(300);

        for _ in 0..300 {
            input.push('a');
        }

        assert_eq!(input.len(), 300);
        assert!(STR0_255::new(input).is_err());
    }

    #[test]
    fn str0_255_str_comparison() {
        let input = "hello";
        let str_255 = STR0_255::new(input).unwrap();

        assert!(str_255 == input.to_string());
        assert!(input.to_string() == str_255);
    }

    #[test]
    fn str0_255_comparison() {
        let a = STR0_255::new("foo").unwrap();
        let b = STR0_255::new("foo").unwrap();
        assert_eq!(a, b);

        let c = STR0_255::new("bar").unwrap();
        assert!(a != c);
    }
}
