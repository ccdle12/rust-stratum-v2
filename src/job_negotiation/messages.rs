use crate::common;
use crate::common::BitFlag;
use crate::error::Result;
use crate::job_negotiation;

/// SetupConnection for the job negotiation sub protocol. This struct restricts
/// the caller to only use feature flags from the job negotiation module.
///
/// This struct has the exact same fields and behaviour as
/// [SetupConnection](../common/messages/SetupConnection)
pub struct SetupConnection(common::SetupConnection);

impl SetupConnection {
    /// Constructor for creating a SetupConnection message for the Mining
    /// sub protocol.
    pub fn new<T: Into<String>>(
        min_version: u16,
        max_version: u16,
        flags: &[job_negotiation::SetupConnectionFlags],
        endpoint_host: T,
        endpoint_port: u16,
        vendor: T,
        hardware_version: T,
        firmware: T,
        device_id: T,
    ) -> Result<common::SetupConnection> {
        let flags: Vec<u8> = flags.iter().map(|x| x.as_byte()).collect();
        common::SetupConnection::new(
            1,
            min_version,
            max_version,
            flags,
            endpoint_host,
            endpoint_port,
            vendor,
            hardware_version,
            firmware,
            device_id,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Serializable;

    #[test]
    fn new_job_negotiation_setup_connection_init() {
        let message = SetupConnection::new(
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
    fn new_job_negotiation_serialize_0() {
        let message = SetupConnection::new(
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
    fn new_job_negotiation_serialize_1() {
        let message = SetupConnection::new(
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
