// use crate::common::types::STR0_255;
//use crate::common::types::U256;
//use crate::common::Protocol;
//use crate::error::{Error, Result};
//use crate::job_negotiation;

//pub struct SetupConnection<T: Into<String>> {
//    /// Used to indicate the protocol the client wants to use on the new connection.
//    pub protocol: Protocol,

//    /// The minimum protocol version the client supports. (current default: 2)
//    pub min_version: u16,

//    /// The maxmimum protocol version the client supports. (current default: 2)
//    pub max_version: u16,

//    /// Flags indicating the optional protocol features the client supports.
//    pub flags: Vec<job_negotiation::SetupConnectionFlags>,

//    /// Used to indicate the hostname or IP address of the endpoint.
//    pub endpoint_host: STR0_255,

//    /// Used to indicate the connecting port value of the endpoint.
//    pub endpoint_port: u16,

//    /// The following fields relay the new_mining device information.
//    ///
//    /// Used to indicate the vendor/manufacturer of the device.
//    pub vendor: T,

//    /// Used to indicate the hardware version of the device.
//    pub hardware_version: T,

//    /// Used to indicate the firmware on the device.
//    pub firmware: T,

//    /// Used to indicate the unique identifier of the device defined by the
//    /// vendor.
//    pub device_id: T,
//}

//impl<T> SetupConnection<T> {
//    pub fn new(
//        min_version: u16,
//        max_version: u16,
//        flags: Vec<mining::SetupConnectionFlags>,
//        endpoint_host: T,
//        endpoint_port: u16,
//        vendor: T,
//        hardware_version: T,
//        firmware: T,
//        device_id: T,
//    ) -> Result<SetupConnection> {
//        let vendor = vendor.into();
//        if *&vendor.is_empty() {
//            return Err(Error::RequirementError(
//                "vendor field in SetupConnection MUST NOT be empty".into(),
//            ));
//        }

//        let firmware = firmware.into();
//        if *&firmware.is_empty() {
//            return Err(Error::RequirementError(
//                "firmware field in SetupConnection MUST NOT be empty".into(),
//            ));
//        }

//        if min_version < 2 {
//            return Err(Error::VersionError("min_version must be atleast 2".into()));
//        }

//        if max_version < 2 {
//            return Err(Error::VersionError("max_version must be atleast 2".into()));
//        }

//        Ok(SetupConnection {
//            protocol: Protocol::JobNegotiation,
//            min_version,
//            max_version,
//            flags,
//            endpoint_host: STR0_255::new(endpoint_host)?,
//            endpoint_port,
//            vendor: STR0_255::new(vendor)?,
//            hardware_version: STR0_255::new(hardware_version)?,
//            firmware: STR0_255::new(firmware)?,
//            device_id: STR0_255::new(device_id)?,
//        })
//    }
//}
