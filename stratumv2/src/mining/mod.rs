// TODO: Replace `` with links
// - What is custom search space splitting?
// - What is difficulty aggregation?
//! This protocol is the direct successor to Stratum V1 and is the only required
//! sub protocol. Mining devices use this protocol to communicate with Upstream
//! Nodes (Proxies or Mining Pools).
//!
//! The protocol has three types of communication channels:
//! - `Standard Channels`: A communication channel with upstream nodes where the
//!                        coinbase transaction and merkle path are not manipulated.
//!
//! - `Extended Channels`: A communication channel allowing more advanced use cases
//!                        such as translation between v1 to v2, difficulty aggregation,
//!                        and `custom search space splitting`
//!
//! - `Group Channels`: A communication channel that is a collection of `Standard Channels`
//!                     opened to a particular connection. The group is addressable
//!                     through a common communication channel.

mod flags;
mod messages;

pub use flags::{SetupConnectionFlags, SetupConnectionSuccessFlags};
pub use messages::{
    OpenExtendedMiningChannel, OpenExtendedMiningChannelError, OpenMiningChannelErrorCodes,
    OpenStandardMiningChannel, OpenStandardMiningChannelError, OpenStandardMiningChannelSuccess,
    SetupConnection, SetupConnectionError, SetupConnectionSuccess,
};
