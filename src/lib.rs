//! A library implementation of the Stratum V2 Protocol.
//!
//! spec: https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit

#![allow(dead_code)]

mod error;
mod util;

/// Common utilities shared between each subprotocol.
pub mod common;

/// Mining subprotocol of Stratum V2.
pub mod mining;
