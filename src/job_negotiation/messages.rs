use crate::common::types::{MessageTypes, STR0_255};
use crate::common::{BitFlag, Deserializable, Framable, Protocol, Serializable};
use crate::error::{Error, Result};
use crate::job_negotiation::SetupConnectionFlags;
use std::borrow::Cow;
use std::{io, str};

// Implementation of the SetupConenction message for the Job Negotiation Protocol.
// use std::borrow::Cow;
impl_setup_connection!(
    Protocol::JobNegotiation,
    SetupConnectionFlags,
    SetupConnection
);

mod tests {
    use super::*;

    #[test]
    fn init_job_negotiation_connection() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresAsyncJobMining]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        );

        assert!(message.is_ok());
    }

    #[test]
    fn serialize_job_negotiation() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[SetupConnectionFlags::RequiresAsyncJobMining]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);
        assert_eq!(buffer[0], 0x01);
        assert_eq!(buffer[5], 0x01);
    }

    #[test]
    fn serialize_job_negotiation_no_flags() {
        let message = SetupConnection::new(
            2,
            2,
            Cow::Borrowed(&[]),
            "0.0.0.0",
            8545,
            "Bitmain",
            "S9i 13.5",
            "braiins-os-2018-09-22-1-hash",
            "some-uuid",
        )
        .unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        let size = message.serialize(&mut buffer).unwrap();

        assert_eq!(size, 75);
        assert_eq!(buffer[0], 0x01);
        assert_eq!(buffer[5], 0x00);
    }
}
