use crate::error::{Error, Result};
use crate::parse::{ByteParser, Deserializable, Serializable};
use std::io;

/// An internal macro that implements a STR0 type that is restricted according to a MAX_LENGTH.
macro_rules! impl_sized_STR0 {
    ($type:ident, $max_length:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $type {
            pub length: u8,
            pub data: String,
        }

        impl $type {
            const MAX_LENGTH: usize = $max_length;

            /// The constructor enforces the String input size as the MAX_LENGTH. A RequirementError
            /// will be returned if the input byte size is greater than the MAX_LENGTH.
            pub fn new<T: Into<String>>(value: T) -> Result<$type> {
                let value = value.into();
                if value.len() > Self::MAX_LENGTH {
                    return Err(Error::RequirementError(
                        "string size cannot be greater than MAX_LENGTH".into(),
                    ));
                }

                let length = value.len() as u8;
                Ok($type {
                    length: length,
                    data: value,
                })
            }
        }

        /// PartialEq implementation allowing direct comparison between the STR0 type and String.
        impl PartialEq<String> for $type {
            fn eq(&self, other: &String) -> bool {
                self.data == *other
            }
        }

        /// PartialEq implementation allowing direct comparison between String and the STR0 type.
        impl PartialEq<$type> for String {
            fn eq(&self, other: &$type) -> bool {
                *self == other.data
            }
        }

        /// From trait implementation that allows a STR0 to be converted into a String.
        impl From<$type> for String {
            fn from(s: $type) -> Self {
                s.data
            }
        }

        /// Deserialize trait implementation that allows a STR0 to be deserialized from a
        /// ByteParser.
        impl Deserializable for $type {
            fn deserialize(parser: &mut ByteParser) -> Result<$type> {
                // Parse the length header before the buffer.
                let header_length = u8::deserialize(parser)?;

                // Then parse the byte buffer.
                let mut data_buffer = vec![];
                data_buffer.extend_from_slice(parser.next_by(header_length.into())?);

                $type::new(String::from_utf8(data_buffer)?)
            }
        }

        /// Serialize trait implementation that allows a STR0 to be serialized into an io::Writer.
        impl Serializable for $type {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                // Write the length header.
                let header_length = self.length.serialize(writer)?;
                // Then write the byte buffer.
                writer.write(self.data.as_bytes())?;

                Ok(header_length + self.length as usize)
            }
        }
    };
}

#[cfg(test)]
macro_rules! impl_sized_STR0_tests {
    ($type:ident, $max_length:expr) => {
        fn make_encoded_str(s: &str) -> Vec<u8> {
            let mut buffer = vec![];
            buffer.push(s.len() as u8);
            buffer.extend_from_slice(s.as_bytes());
            return buffer;
        }

        fn make_decoded_str(s: &str) -> $type {
            $type {
                length: s.len() as u8,
                data: s.into(),
            }
        }

        #[test]
        fn new() {
            let empty: String = "".into();
            assert_eq!(
                $type::new("").unwrap(),
                $type {
                    length: 0,
                    data: empty
                }
            );

            let nonempty: String = "human-readable-data".into();
            assert_eq!(
                $type::new("human-readable-data").unwrap(),
                $type {
                    length: 19,
                    data: nonempty
                }
            );

            let max_length: String = (0..$max_length).map(|_| 'a').collect();
            assert_eq!(
                $type::new(max_length.clone()).unwrap(),
                $type {
                    length: $max_length,
                    data: max_length
                }
            );

            let over_limit: String = (0..$max_length + 1).map(|_| 'a').collect();
            assert!(matches!(
                $type::new(over_limit),
                Err(Error::RequirementError { .. })
            ));
        }

        #[test]
        fn serde_ok_empty() {
            let encoded = make_encoded_str("");
            let decoded = make_decoded_str("");
            assert_eq!(deserialize::<$type>(&encoded).unwrap(), decoded);
            assert_eq!(serialize(&decoded).unwrap(), encoded);
        }

        #[test]
        fn serde_ok_nonempty() {
            let encoded = make_encoded_str("valid data");
            let decoded = make_decoded_str("valid data");
            assert_eq!(deserialize::<$type>(&encoded).unwrap(), decoded);
            assert_eq!(serialize(&decoded).unwrap(), encoded);
        }

        #[test]
        fn deserialize_err() {
            // No data to deserialize.
            assert!(matches!(
                deserialize::<$type>(&[]),
                Err(Error::ParseError { .. })
            ));
            // No data after promised length.
            assert!(matches!(
                deserialize::<$type>(&[1u8]),
                Err(Error::ParseError { .. })
            ));
            // Insufficient data after promised length.
            assert!(matches!(
                deserialize::<$type>(&[2u8, 42u8]),
                Err(Error::ParseError { .. })
            ));
            // Non-Utf8 data.
            let data: [u8; 7] = [0x06, 0xed, 0xa0, 0x80, 0xed, 0xb0, 0x80];
            assert!(matches!(
                deserialize::<$type>(&data),
                Err(Error::FromUtf8Error { .. })
            ));
        }
    };
}

// TODO(chpatton013): The mention of STR0_32 is probably an error in the specification. Anticipating
// rename to STR0_31.
impl_sized_STR0!(STR0_32, 32);
impl_sized_STR0!(STR0_255, 255);

#[cfg(test)]
mod str0_32_tests {
    use super::*;
    use crate::parse::{deserialize, serialize};

    impl_sized_STR0_tests!(STR0_32, 32);

    #[test]
    fn deserialize_over_max_length() {
        // The type used to encode the length of this payload has the necessary domain to describe
        // payloads much longer than the supposed maximum.
        let data = make_encoded_str((0..33).map(|_| 'a').collect::<String>().as_str());
        assert!(matches!(
            deserialize::<STR0_32>(data.as_slice()),
            Err(Error::RequirementError { .. })
        ));
    }
}

#[cfg(test)]
mod str0_255_tests {
    use super::*;
    use crate::parse::{deserialize, serialize};

    impl_sized_STR0_tests!(STR0_255, 255);
}
