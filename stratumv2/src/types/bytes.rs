use crate::codec::{ByteParser, Deserializable, Serializable};
use crate::error::{Error, Result};
use crate::types::U24;
use std::io;

/// An internal macro that implements a B0 type that is restricted according to a MAX_LENGTH.
macro_rules! impl_sized_B0 {
    ($type:ident, $length_type:ident) => {
        impl_sized_B0!($type, $length_type, $length_type::MAX as usize);
    };
    ($type:ident, $length_type:ident, $max_length:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $type {
            pub length: $length_type,
            pub data: Vec<u8>,
        }

        impl $type {
            const MAX_LENGTH: usize = $max_length;

            /// The constructor enforces the Vec<u8> input size as the MAX_LENGTH. A
            /// RequirementError will be returned if the input byte size is greater than the
            /// MAX_LENGTH.
            pub fn new<T: Into<Vec<u8>>>(value: T) -> Result<$type> {
                let value = value.into();
                if value.len() > Self::MAX_LENGTH {
                    return Err(Error::RequirementError(
                        "bytes size cannot be greater than MAX_LENGTH".into(),
                    ));
                }

                use std::convert::TryInto;
                Ok($type {
                    length: value.len().try_into().unwrap(),
                    data: value,
                })
            }
        }

        /// PartialEq implementation allowing direct comparison between the B0 type and Vec<u8>.
        impl PartialEq<Vec<u8>> for $type {
            fn eq(&self, other: &Vec<u8>) -> bool {
                self.data == *other
            }
        }

        /// PartialEq implementation allowing direct comparison between Vec<u8> and the B0 type.
        impl PartialEq<$type> for Vec<u8> {
            fn eq(&self, other: &$type) -> bool {
                *self == other.data
            }
        }

        /// From trait implementation that allows a B0 to be converted into a Vec<u8>.
        impl From<$type> for Vec<u8> {
            fn from(b: $type) -> Self {
                b.data
            }
        }

        /// Deserialize trait implementation that allows a B0 to be deserialized from a ByteParser.
        impl Deserializable for $type {
            fn deserialize(parser: &mut ByteParser) -> Result<$type> {
                // Parse the length header before the buffer.
                let header_length = $length_type::deserialize(parser)?;
                // Then parse the byte buffer.
                let bytes = parser.next_by(header_length.clone().into())?;

                $type::new(bytes)
            }
        }

        /// Serialize trait implementation that allows a B0 to be serialized into an io::Writer.
        impl Serializable for $type {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                // Write the length header.
                let header_length = self.length.serialize(writer)?;
                // Then write the byte buffer.
                writer.write(self.data.as_slice())?;

                Ok(header_length + self.data.len())
            }
        }
    };
}

#[cfg(test)]
macro_rules! impl_sized_B0_tests {
    ($type:ident, $length_type:ident) => {
        impl_sized_B0_tests!($type, $length_type, $length_type::MAX as usize);
    };
    ($type:ident, $length_type:ident, $max_length:expr) => {
        fn make_encoded_bytes(payload: &[u8]) -> Vec<u8> {
            use std::convert::TryInto;

            let length = payload.len().try_into().unwrap();
            let header = serialize::<$length_type>(&length).unwrap();

            let mut buffer = vec![];
            buffer.extend_from_slice(header.as_slice());
            buffer.extend_from_slice(payload);
            return buffer;
        }

        fn make_decoded_bytes(s: &[u8]) -> $type {
            use std::convert::TryInto;
            $type {
                length: s.len().try_into().unwrap(),
                data: s.into(),
            }
        }

        #[test]
        fn new_empty() {
            use std::convert::TryInto;
            let data = vec![];
            let length: $length_type = 0usize.try_into().unwrap();
            assert_eq!(
                $type::new(data.clone()).unwrap(),
                $type {
                    length: length,
                    data: data
                }
            );
        }

        #[test]
        fn new_nonempty() {
            use std::convert::TryInto;
            let data = vec![1, 2, 3, 4, 5];
            let length: $length_type = 5usize.try_into().unwrap();
            assert_eq!(
                $type::new(data.clone()).unwrap(),
                $type {
                    length: length,
                    data: data
                }
            );
        }

        #[test]
        fn new_max_length() {
            // There's no need to test the larger variants of this macro. If it works for the
            // smaller sizes it will work for the larger ones as well.
            if $max_length < 100000 {
                use std::convert::TryInto;
                let data: Vec<u8> = [0; $max_length].into();
                let length: $length_type = $max_length.try_into().unwrap();
                assert_eq!(
                    $type::new(data.clone()).unwrap(),
                    $type {
                        length: length,
                        data: data
                    }
                );
            }
        }

        #[test]
        fn new_over_limit() {
            // There's no need to test the larger variants of this macro. If it works for the
            // smaller sizes it will work for the larger ones as well.
            if $max_length < 100000 {
                let data: Vec<u8> = [0; $max_length + 1].into();
                assert!(matches!(
                    $type::new(data),
                    Err(Error::RequirementError { .. })
                ));
            }
        }

        #[test]
        fn serde_ok_empty() {
            let encoded = make_encoded_bytes(&[]);
            let decoded = make_decoded_bytes(&[]);
            assert_eq!(deserialize::<$type>(&encoded).unwrap(), decoded);
            assert_eq!(serialize(&decoded).unwrap(), encoded);
        }

        #[test]
        fn serde_ok_nonempty() {
            let encoded = make_encoded_bytes(&[1, 2, 3, 4, 5]);
            let decoded = make_decoded_bytes(&[1, 2, 3, 4, 5]);
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
        }
    };
}

// TODO(chpatton013): The mention of both B0_31 and B0_32 is probably an error in the
// specification. One of those (anticipating B0_32) will most-likely be removed.
impl_sized_B0!(B0_31, u8, 31);
impl_sized_B0!(B0_32, u8, 32);
impl_sized_B0!(B0_255, u8);
impl_sized_B0!(B0_64K, u16);
impl_sized_B0!(B0_16M, U24);

#[cfg(test)]
mod b0_31_tests {
    use super::*;
    use crate::codec::{deserialize, serialize};

    impl_sized_B0_tests!(B0_31, u8, 31);

    #[test]
    fn deserialize_over_max_length() {
        // The type used to encode the length of this payload has the necessary domain to describe
        // payloads much longer than the supposed maximum.
        let data = make_encoded_bytes(&[0; 32]);
        assert!(matches!(
            deserialize::<B0_31>(data.as_slice()),
            Err(Error::RequirementError { .. })
        ));
    }
}

#[cfg(test)]
mod b0_32_tests {
    use super::*;
    use crate::codec::{deserialize, serialize};

    impl_sized_B0_tests!(B0_32, u8, 32);

    #[test]
    fn deserialize_over_max_length() {
        // The type used to encode the length of this payload has the necessary domain to describe
        // payloads much longer than the supposed maximum.
        let data = make_encoded_bytes(&[0; 33]);
        assert!(matches!(
            deserialize::<B0_32>(data.as_slice()),
            Err(Error::RequirementError { .. })
        ));
    }
}

#[cfg(test)]
mod b0_255_tests {
    use super::*;
    use crate::codec::{deserialize, serialize};

    impl_sized_B0_tests!(B0_255, u8);
}

#[cfg(test)]
mod b0_64_k_tests {
    use super::*;
    use crate::codec::{deserialize, serialize};

    impl_sized_B0_tests!(B0_64K, u16);
}

#[cfg(test)]
mod b0_16_m_tests {
    use super::*;
    use crate::codec::{deserialize, serialize};

    impl_sized_B0_tests!(B0_16M, U24);
}
