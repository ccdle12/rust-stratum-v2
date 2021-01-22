use crate::common::messages::RawSetupConnection;
use crate::common::{BitFlag, Deserializable, Framable, Protocol, Serializable};
use crate::error::{Error, Result};
use crate::job_negotiation;
use crate::job_negotiation::SetupConnectionFlags;
use std::{io, str};

pub struct JobNegotiationSetupConnection {
    internal: RawSetupConnection,
}

impl_setup_connection!(
    Protocol::JobNegotiation,
    SetupConnectionFlags,
    JobNegotiationSetupConnection
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::Serializable;
    use crate::job_negotiation;

    #[test]
    fn init_job_negotiation_connection() {
        let message = JobNegotiationSetupConnection::new(
            2,
            2,
            &[job_negotiation::SetupConnectionFlags::RequiresAsyncJobMining],
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
        let message = JobNegotiationSetupConnection::new(
            2,
            2,
            &[job_negotiation::SetupConnectionFlags::RequiresAsyncJobMining],
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
        let message: JobNegotiationSetupConnection = JobNegotiationSetupConnection::new(
            2,
            2,
            &[],
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
