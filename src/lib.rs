//! A library implementation of the Stratum V2 Protocol.
//!
//! Stratum V2 sources:
//! - [Stratum V2 Overview](https://braiins.com/stratum-v2)
//! - [Stratum V2 Specification](https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit)

#![allow(dead_code)]

mod error;
mod util;

/// Common utilities shared between each subprotocol.
pub mod common;

/// Mining subprotocol of Stratum V2.
pub mod mining;

/// Job Negotiation subprotocol of Stratum V2.
pub mod job_negotiation;
