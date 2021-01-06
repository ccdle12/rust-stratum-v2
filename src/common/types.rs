use crate::error::Error::RequirementError;
use crate::error::Result;

/// U256 is an unsigned integer type of 256-bits in little endian. This will
/// usually be used to represent a raw SHA256 byte output.
pub(crate) type U256 = [u8; 32];

/// STR0_255 is a struct that contains a String limited to a maximum of 255 bytes.
/// The byte representation will contain a <1 byte length prefix + variable length STR0_255>.
pub struct STR0_255(String);

impl STR0_255 {
    /// Constructor for the STR0_255 struct. The constructor enforces the String
    /// input size as 255 bytes. A RequirementError will be returned if
    /// the input byte size is greater than 255.
    pub fn new<T: Into<String>>(value: T) -> Result<STR0_255> {
        let value = value.into();
        if value.len() > 255 {
            return Err(RequirementError(
                "string size cannot be greater than 255".into(),
            ));
        }

        Ok(STR0_255(value))
    }

    /// Returns the byte representation of the STR0_255. Specifically
    /// it returns the byte representation for serializing according to the
    /// protocol specification which is <1 byte length prefix + variable length STR0_255>.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![self.0.len() as u8];
        buffer.extend_from_slice(self.0.as_bytes());

        buffer
    }
}

/// From trait implementation that allows a STR0_255 to be converted into a
/// String.
impl From<STR0_255> for String {
    fn from(s: STR0_255) -> Self {
        s.0
    }
}

/// MessageTypes contain all the variations for the byte representation of a
/// messages used in a message frame.
pub(crate) enum MessageTypes {
    SetupConnection,
    SetupConnectionSuccess,
    SetupConnectionError,
}

impl From<MessageTypes> for u8 {
    fn from(m: MessageTypes) -> Self {
        match m {
            MessageTypes::SetupConnection => 0x00,
            MessageTypes::SetupConnectionSuccess => 0x01,
            MessageTypes::SetupConnectionError => 0x03,
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
}
