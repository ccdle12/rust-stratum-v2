use crate::common::messages::RawSetupConnection;
use crate::common::{BitFlag, Deserializable, Framable, Protocol, Serializable};
use crate::error::{Error, Result};
use crate::job_negotiation;
use crate::job_negotiation::SetupConnectionFlags;
use std::{io, str};

pub struct SetupConnection {
    internal: RawSetupConnection,
}

impl SetupConnection {
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
    ) -> Result<SetupConnection> {
        let flags = flags
            .iter()
            .map(|x| x.as_bit_flag())
            .fold(0, |acc, byte| (acc | byte));

        let internal = RawSetupConnection::new(
            Protocol::JobNegotiation,
            min_version,
            max_version,
            flags,
            endpoint_host.into(),
            endpoint_port,
            vendor.into(),
            hardware_version.into(),
            firmware.into(),
            device_id.into(),
        )?;

        Ok(SetupConnection { internal })
    }

    fn min_version(&self) -> u16 {
        self.internal.min_version
    }

    fn max_version(&self) -> u16 {
        self.internal.max_version
    }

    fn flags(&self) -> Vec<job_negotiation::SetupConnectionFlags> {
        SetupConnectionFlags::deserialize_flags(self.internal.flags)
    }

    fn endpoint_host(&self) -> &str {
        &self.internal.endpoint_host.0
    }

    fn endpoint_port(&self) -> u16 {
        self.internal.endpoint_port
    }

    fn vendor(&self) -> &str {
        &self.internal.vendor.0
    }

    fn hardware_version(&self) -> &str {
        &self.internal.hardware_version.0
    }

    fn firmware(&self) -> &str {
        &self.internal.firmware.0
    }

    fn device_id(&self) -> &str {
        &self.internal.device_id.0
    }
}

/// Implementation of the Serializable trait to serialize the contents
/// of the SetupConnection message to the valid message format.
impl Serializable for SetupConnection {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok(writer.write(&self.internal.serialize())?)
    }
}

// TODO: Docstring
impl Deserializable for SetupConnection {
    fn deserialize(bytes: &[u8]) -> Result<SetupConnection> {
        Ok(SetupConnection {
            internal: RawSetupConnection::deserialize(bytes)?,
        })
    }
}

/// Implementation of the Framable trait to build a network frame for the
/// SetupConnection message.
impl Framable for SetupConnection {
    fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok(writer.write(&self.internal.frame())?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::Serializable;
    use crate::job_negotiation;

    #[test]
    fn init_job_negotiation_connection() {
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
    fn serialize_job_negotiation() {
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
    fn serialize_job_negotiation_no_flags() {
        let message: SetupConnection = SetupConnection::new(
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
