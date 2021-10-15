//! Codec contains all the functionality to serialize, deserialize and frame network messages.
//! This also includes the required Serializable, Deserializable and Frameable traits.

// mod error;
mod frame;

pub use frame::CHANNEL_BIT_MASK;
pub use frame::{frame, unframe, Frameable, Message};
