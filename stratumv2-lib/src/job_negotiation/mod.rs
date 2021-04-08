//! The sub protocol allows a Job Negotiator to intermediate between Upstream
//! Nodes (Mining Pools), Bitcoind and Proxies/Mining Devices. The protocol
//! allows a block template to be negotiated with a mining pool, including the
//! transaction set.
//!
//! The results of the job negotiation with a Mining Pool means downstream
//! nodes (Mining Farms/Devices) can use the same negotiation result on all
//! their connections.

pub mod setup_connection;
pub mod setup_connection_error;
pub mod setup_connection_success;

pub use setup_connection::{SetupConnection, SetupConnectionFlags};
pub use setup_connection_error::SetupConnectionError;
pub use setup_connection_success::{SetupConnectionSuccess, SetupConnectionSuccessFlags};
