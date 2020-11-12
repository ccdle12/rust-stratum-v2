use crate::error::Result;

/// Convert a string to a string limited to 255 bytes.
pub fn string_to_str0_255<T: Into<String>>(input: T) -> Result<String> {
    Ok(std::str::from_utf8(&trim_to_255_bytes(input))?.to_string())
}

/// Convert a string to byte representation of STR0_255.
/// The byte represenation is <1 byte length of string><string 0..255>.
pub fn string_to_str0_255_bytes<T: Into<String>>(input: T) -> Result<Vec<u8>> {
    let bytes = trim_to_255_bytes(input);

    let mut buffer = vec![bytes.len() as u8];
    buffer.extend_from_slice(bytes.as_slice());

    Ok(buffer)
}

// Helper function to return 0..255 bytes of a string.
fn trim_to_255_bytes<T: Into<String>>(input: T) -> Vec<u8> {
    let mut bytes = input.into().as_bytes().to_vec();

    if bytes.len() > 255 {
        bytes = bytes[0..255].to_vec();
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str0_255_init() {
        assert!(string_to_str0_255("hello").is_ok())
    }

    #[test]
    fn str0_255_to_bytes() {
        let input = "hello";
        let bytes = string_to_str0_255_bytes(input).unwrap();

        let expected = vec![0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        assert!(bytes == expected);

        let invalid = vec![0x68, 0x65, 0x6c];
        assert!(bytes != invalid);
    }

    #[test]
    fn str0_255_size_limit() {
        let mut input = String::with_capacity(510);

        for _ in 0..510 {
            input.push('a');
        }
        assert_eq!(input.len(), 510);

        let output = string_to_str0_255(input).unwrap();
        assert_eq!(output.len(), 255);
    }

    #[test]
    fn str0_255_to_bytes_size_limit() {
        let mut input = String::with_capacity(510);

        for _ in 0..510 {
            input.push('a');
        }
        assert_eq!(input.len(), 510);

        let output = string_to_str0_255_bytes(input).unwrap();
        assert_eq!(output[0], 0xFF);
        assert_eq!(output.len(), 256);
    }
}
