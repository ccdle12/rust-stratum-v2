//! A library implementation of the Stratum V2 Protocol.
//!
//! Stratum V2 sources:
//! - [Stratum V2 Overview](https://braiins.com/stratum-v2)
//! - [Stratum V2 Specification](https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit)

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate thiserror;

pub mod common;
/// Errors returned in the library.
pub mod error;
pub mod frame;
pub mod job_negotiation;
mod macro_message;
pub mod mining;
pub mod parse;
pub mod types;
