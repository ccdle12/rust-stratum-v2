//! A library implementation of the Stratum V2 Protocol.
//!
//! Stratum V2 sources:
//! - [Stratum V2 Overview](https://braiins.com/stratum-v2)
//! - [Stratum V2 Specification](https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit)

#![allow(dead_code)]

/// Errors returned in the library.
mod error;

/// Utility functions that help with generating messages or other Stratum V2
/// functionality.
pub mod util;

/// Common messages and utilities for each sub protocol.
pub mod common;

/// Mining is the main sub protocol of Stratum V2.
pub mod mining;

/// Job Negotiation is a sub protocol of Stratum V2.
pub mod job_negotiation;
