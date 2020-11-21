//! The sub protocol allows a Job Negotiator to intermediate between Upstream
//! Nodes (Mining Pools), Bitcoind and Proxies/Mining Devices. The protocol
//! allows a block template to be negotiated with a mining pool, including the
//! transaction set.
//!
//! The results of the job negotiation with a Mining Pool means downstream
//! nodes (Mining Farms/Devices) can use the same negotiation result on all
//! their connections.

/// Job Negotiation subprotocol messages.
pub mod messages;
