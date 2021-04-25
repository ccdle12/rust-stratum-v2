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

mod open_extended_mining_channel;
mod open_extended_mining_channel_error;
mod open_extended_mining_channel_success;
mod open_mining_channel_error;
mod open_standard_mining_channel;
mod open_standard_mining_channel_error;
mod open_standard_mining_channel_success;
mod setup_connection;
mod setup_connection_error;
mod setup_connection_success;

pub use open_extended_mining_channel::OpenExtendedMiningChannel;
pub use open_extended_mining_channel_error::OpenExtendedMiningChannelError;
pub use open_extended_mining_channel_success::OpenExtendedMiningChannelSuccess;
pub use open_mining_channel_error::OpenMiningChannelErrorCode;
pub use open_standard_mining_channel::OpenStandardMiningChannel;
pub use open_standard_mining_channel_error::OpenStandardMiningChannelError;
pub use open_standard_mining_channel_success::OpenStandardMiningChannelSuccess;
pub use setup_connection::{SetupConnection, SetupConnectionFlags};
pub use setup_connection_error::SetupConnectionError;
pub use setup_connection_success::{SetupConnectionSuccess, SetupConnectionSuccessFlags};
