use crate::error::{Error, Result};
use crate::frame::Frameable;
use crate::job_negotiation;
use crate::mining;
use crate::parse::{ByteParser, Deserializable, Serializable};
use crate::types::MessageType;
// use crate::template_distribution;
// use crate::job_distribution;
use std::convert::TryFrom;
use std::io;
/// Protocol is an enum representing each sub protocol of Stratum V2.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Protocol {
    /// Mining is the main and only required sub protocol in Stratum V2.
    Mining,

    /// JobNegotiation is a protocol for intermediate nodes to broker
    /// the terms of a connection between downstream nodes and upstream nodes.
    JobNegotiation,

    /// TemplateDistribution is a protocol for getting the next block from the
    /// Bitcoin RPC. This protocol is intented to replace getblocktemplate.
    TemplateDistribution,

    /// JobDistribution is a protocol for passing newly-negotiated work from the
    /// Job Negotiator to proxies or mining devices. If miners aren't choosing
    /// their transaction sets, then jobs will be distributed from pools directly
    /// to proxies/mining devices.
    JobDistribution,
}

impl From<&Protocol> for u8 {
    fn from(protocol: &Protocol) -> Self {
        match protocol {
            Protocol::Mining => 0,
            Protocol::JobNegotiation => 1,
            Protocol::TemplateDistribution => 2,
            Protocol::JobDistribution => 3,
        }
    }
}

impl From<Protocol> for u8 {
    fn from(protocol: Protocol) -> Self {
        u8::from(&protocol)
    }
}

impl TryFrom<u8> for Protocol {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            0 => Ok(Protocol::Mining),
            1 => Ok(Protocol::JobNegotiation),
            2 => Ok(Protocol::TemplateDistribution),
            3 => Ok(Protocol::JobDistribution),
            // TODO(chpatton013): Pick an error type that is more context-agnostic.
            _ => Err(Error::DeserializationError(
                "received unknown protocol byte in setup connection message".into(),
            )),
        }
    }
}

impl Serializable for Protocol {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok(u8::from(self).serialize(writer)?)
    }
}

impl Deserializable for Protocol {
    fn deserialize(parser: &mut ByteParser) -> Result<Protocol> {
        Protocol::try_from(u8::deserialize(parser)?)
    }
}

/// Contains all the variants of each subprotocols SetupConnection message.
/// When constructing a NetworkMessage this enum should be used to correctly
/// serialize the SetupConnection specific to the subprotocol.
pub enum SetupConnection {
    Mining(mining::SetupConnection),
    JobNegotiation(job_negotiation::SetupConnection),
    // TemplateDistribution(template_distribution::SetupConnection),
    // JobDistribution(job_distribution::SetupConnection),
}

impl SetupConnection {
    /// SetupConnection message for the mining subprotocol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stratumv2_lib::mining::SetupConnectionFlags;
    /// use stratumv2_lib::common::SetupConnection;
    ///
    /// let new_connection = SetupConnection::new_mining(
    ///    2,
    ///    2,
    ///    SetupConnectionFlags::REQUIRES_STANDARD_JOBS | SetupConnectionFlags::REQUIRES_VERSION_ROLLING,
    ///    "0.0.0.0",
    ///    8545,
    ///    "Bitmain",
    ///    "S9i 13.5",
    ///    "braiins-os-2018-09-22-1-hash",
    ///    "some-device-uuid",
    /// );
    /// assert!(new_connection.is_ok());
    pub fn new_mining<T: Into<String>>(
        min_version: u16,
        max_version: u16,
        flags: mining::SetupConnectionFlags,
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection> {
        Ok(SetupConnection::Mining(mining::SetupConnection::new(
            min_version,
            max_version,
            flags,
            endpoint_host,
            endpoint_port,
            vendor,
            hardware_version,
            firmware,
            device_id,
        )?))
    }

    /// SetupConnection message for the job negotiation subprotocol.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stratumv2_lib::job_negotiation::SetupConnectionFlags;
    /// use stratumv2_lib::common::SetupConnection;
    ///
    /// let new_connection = SetupConnection::new_job_negotation(
    ///    2,
    ///    2,
    ///    SetupConnectionFlags::REQUIRES_ASYNC_JOB_MINING,
    ///    "0.0.0.0",
    ///    8545,
    ///    "Bitmain",
    ///    "S9i 13.5",
    ///    "braiins-os-2018-09-22-1-hash",
    ///    "some-device-uuid",
    /// );
    /// assert!(new_connection.is_ok());
    pub fn new_job_negotation<T: Into<String>>(
        min_version: u16,
        max_version: u16,
        flags: job_negotiation::SetupConnectionFlags,
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<SetupConnection> {
        Ok(SetupConnection::JobNegotiation(
            job_negotiation::SetupConnection::new(
                min_version,
                max_version,
                flags,
                endpoint_host,
                endpoint_port,
                vendor,
                hardware_version,
                firmware,
                device_id,
            )?,
        ))
    }
}

impl Serializable for SetupConnection {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        let length = match self {
            SetupConnection::Mining(v) => {
                Protocol::Mining.serialize(writer)? + v.serialize(writer)?
            }
            SetupConnection::JobNegotiation(v) => {
                Protocol::JobNegotiation.serialize(writer)? + v.serialize(writer)?
            } // SetupConnection::TemplateDistribution(v) => {
              //     Protocol::TemplateDistribution.serialize(writer)? + v.serialize(writer)?
              // }
              // SetupConnection::JobDistribution(v) => {
              //     Protocol::JobDistribution.serialize(writer)? + v.serialize(writer)?
              // }
        };

        Ok(length)
    }
}

impl Deserializable for SetupConnection {
    fn deserialize(parser: &mut ByteParser) -> Result<SetupConnection> {
        let protocol = Protocol::deserialize(parser)?;
        let variant = match protocol {
            Protocol::Mining => {
                SetupConnection::Mining(mining::SetupConnection::deserialize(parser)?)
            }
            Protocol::JobNegotiation => SetupConnection::JobNegotiation(
                job_negotiation::SetupConnection::deserialize(parser)?,
            ),
            _ => return Err(Error::Unimplemented()),
            // Protocol::TemplateDistribution => SetupConnection::TemplateDistribution(
            //     template_distribution::SetupConnection::deserialize(parser)?,
            // ),
            // Protocol::JobDistribution => SetupConnection::JobDistribution(
            //     job_distribution::SetupConnection::deserialize(parser)?,
            // ),
        };

        Ok(variant)
    }
}

impl Frameable for SetupConnection {
    fn message_type() -> MessageType {
        MessageType::SetupConnection
    }
}

#[cfg(test)]
macro_rules! impl_setup_connection_tests {
    ($protocol:expr, $fn:expr, $flags:ident) => {
        use crate::frame::frame;
        use crate::parse;
        use crate::types::U24;
        use std::collections::HashMap;

        fn default_setup_conn(
            empty: bool,
            args: HashMap<String, String>,
        ) -> Result<SetupConnection> {
            let mut min_version = 2;
            let mut max_version = 2;
            let mut vendor = "Bitmain";
            let mut firmware = "braiins-os-2018-09-22-1-hash";

            if args.contains_key("min_version") {
                min_version = args.get("min_version").unwrap().parse::<u16>().unwrap();
            }

            if args.contains_key("max_version") {
                max_version = args.get("max_version").unwrap().parse::<u16>().unwrap();
            }

            if args.contains_key("vendor") {
                vendor = args.get("vendor").unwrap();
            }

            if args.contains_key("firmware") {
                firmware = args.get("firmware").unwrap();
            }

            let flags = if empty {
                $flags::empty()
            } else {
                $flags::all()
            };

            $fn(
                min_version,
                max_version,
                flags,
                "0.0.0.0",
                8545,
                vendor,
                "S9u 13.5",
                firmware,
                "some-uuid",
            )
        }

        #[test]
        fn constructor_errors() {
            // Check that empty vendor string should return an error.
            let mut args = HashMap::new();
            args.insert("vendor".into(), "".into());
            assert!(default_setup_conn(false, args).is_err());

            // Check that empty firmware string should return an error.
            let mut args = HashMap::new();
            args.insert("firmware".into(), "".into());
            assert!(default_setup_conn(false, args).is_err());

            // Check that min and max versions must be atleast 2.
            let mut args = HashMap::new();
            args.insert("min_version".into(), "1".into());
            assert!(default_setup_conn(false, args).is_err());

            let mut args = HashMap::new();
            args.insert("max_version".into(), "1".into());
            assert!(default_setup_conn(false, args).is_err());
        }

        #[test]
        fn serialize() {
            let conn = default_setup_conn(false, HashMap::new()).unwrap();
            let result = parse::serialize(&conn).unwrap();

            // Check the serialized connection is the correct length.
            assert_eq!(result.len(), 75);

            // Check the protocol byte was serialized correctly.
            assert_eq!(result[0], $protocol.into());

            // Check the flags were serialized correctly.
            assert_eq!(result[5..9], parse::serialize(&$flags::all()).unwrap());

            // Sanity check - deserializing back to the struct does not cause
            // errors.
            assert!(parse::deserialize::<SetupConnection>(&result).is_ok());
        }

        #[test]
        fn serialize_empty_flags() {
            let conn = default_setup_conn(true, HashMap::new()).unwrap();
            let result = parse::serialize(&conn).unwrap();

            // Check the optional flags still serialize but to empty values.
            assert_eq!(result[5..9], [0u8; 4]);
        }

        #[test]
        fn frame_message() {
            let conn = default_setup_conn(false, HashMap::new()).unwrap();
            let network_message = frame(&conn).unwrap();

            let result = parse::serialize(&network_message).unwrap();
            assert_eq!(result.len(), 81);

            // Check the extension type is empty.
            assert_eq!(result[0..2], [0u8; 2]);

            // Check that the correct byte for the message type was used.
            assert_eq!(result[2], network_message.message_type.msg_type());

            // Check that the correct message length was used.
            assert_eq!(
                parse::deserialize::<U24>(&result[3..6]).unwrap(),
                network_message.payload.len() as u32
            );
        }
    };
}

#[cfg(test)]
mod mining_setup_connection_tests {
    use super::*;
    use crate::mining::SetupConnectionFlags;

    impl_setup_connection_tests!(
        Protocol::Mining,
        SetupConnection::new_mining,
        SetupConnectionFlags
    );
}

#[cfg(test)]
mod job_negotiation_setup_connection_tests {
    use super::*;
    use crate::job_negotiation::SetupConnectionFlags;

    impl_setup_connection_tests!(
        Protocol::JobNegotiation,
        SetupConnection::new_job_negotation,
        SetupConnectionFlags
    );
}
