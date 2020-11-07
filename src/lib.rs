//! A library implementation of the Stratum V2 Protocol.
//!
//! spec: https://docs.google.com/document/d/1FadCWj-57dvhxsnFM_7X806qyvhR0u3i85607bGHxvg/edit

#![allow(dead_code)]

mod common_messages;
mod error;

pub use common_messages::*;
pub use error::{Error, Result};
