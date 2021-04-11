use crate::error::{Error, Result};
use crate::job_negotiation;
use crate::mining;
use crate::parse::{ByteParser, Deserializable, Serializable};
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
        let length = u8::from(self).serialize(writer)?;

        Ok(length)
    }
}

impl Deserializable for Protocol {
    fn deserialize(parser: &mut ByteParser) -> Result<Protocol> {
        Protocol::try_from(u8::deserialize(parser)?)
    }
}

pub enum SetupConnection {
    Mining(mining::SetupConnection),
    JobNegotiation(job_negotiation::SetupConnection),
    // TemplateDistribution(template_distribution::SetupConnection),
    // JobDistribution(job_distribution::SetupConnection),
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
