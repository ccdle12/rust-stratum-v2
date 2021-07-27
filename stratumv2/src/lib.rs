//! A library implementation of the Stratum V2 Protocol.
//!
//! Stratum V2 sources:
//! - [Stratum V2 Overview](https://braiins.com/stratum-v2)
//! - [Stratum V2 Specification](https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit)

#[macro_use]
extern crate bitflags;
extern crate thiserror;

pub extern crate bitcoin;

/// Common messages and flags for all sub protocols.
pub mod common;

/// Errors returned in the library.
pub mod error;

/// Job Negotiation is a sub protocol of Stratum V2.
pub mod job_negotiation;
mod macro_message;

/// Mining is the main sub protocol of Stratum V2.
pub mod mining;

/// Codec contains all the functionality to serialize, deserialize and frame network messages.
/// This also includes the required Serializable, Deserializable and Frameable traits.
pub mod codec;

/// Types used in all Stratum V2 Protocols.
pub mod types;

/// Noise contains all the required messages and functions to perform the Noise
/// Handshake, creating a symmetric key to perform secure communication.
/// This module contains functions to verify and generate signatures
/// for both Client and Server to attest to the authenticty of an Upstream Node.
pub mod noise;

/// Structs and Traits required for a networked implementation
pub mod network;
