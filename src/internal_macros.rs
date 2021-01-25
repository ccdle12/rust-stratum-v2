/// A convenience macro for serializing a variable length of byte slices to a
/// Vector.
macro_rules! serialize {
    ($($x:expr),*) => {{
        let mut buffer: Vec<u8> = Vec::new();
        $( buffer.extend_from_slice($x);)*
        buffer
    }};
}

/// Implemention of the requirements for a SetupConnection message for each
/// sub protocol.
macro_rules! impl_setup_connection {
    ($protocol:expr, $flags:ident, $conn_type:ident) => {
        /// SetupConnection is the first message sent by a client on a new connection.
        ///
        /// The SetupConnection struct contains all the common fields for the
        /// SetupConnection message for each Stratum V2 subprotocol.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::borrow::Cow;
        /// use stratumv2::mining;
        /// use stratumv2::job_negotiation;
        ///
        /// let mining_connection = mining::SetupConnection::new(
        ///    2,
        ///    2,
        ///    Cow::Borrowed(&[
        ///        mining::SetupConnectionFlags::RequiresStandardJobs,
        ///        mining::SetupConnectionFlags::RequiresVersionRolling
        ///     ]),
        ///    "0.0.0.0",
        ///    8545,
        ///    "Bitmain",
        ///    "S9i 13.5",
        ///    "braiins-os-2018-09-22-1-hash",
        ///    "some-device-uuid",
        /// );
        /// assert!(mining_connection.is_ok());
        /// assert_eq!(
        ///     mining_connection.unwrap().flags[0],
        ///     mining::SetupConnectionFlags::RequiresStandardJobs
        /// );
        ///
        /// let job_negotiation_connection = job_negotiation::SetupConnection::new(
        ///    2,
        ///    2,
        ///    Cow::Borrowed(&[
        ///        job_negotiation::SetupConnectionFlags::RequiresAsyncJobMining,
        ///     ]),
        ///    "0.0.0.0",
        ///    8545,
        ///    "Bitmain",
        ///    "S9i 13.5",
        ///    "braiins-os-2018-09-22-1-hash",
        ///    "some-device-uuid",
        /// );
        /// assert!(job_negotiation_connection.is_ok());
        /// ```
        #[derive(Debug, Clone)]
        pub struct $conn_type<'a> {
            /// Used to indicate the protocol the client wants to use on the new connection.
            protocol: Protocol,

            /// The minimum protocol version the client supports. (current default: 2)
            pub min_version: u16,

            /// The maxmimum protocol version the client supports. (current default: 2)
            pub max_version: u16,

            /// Flags indicating the optional protocol features the client supports.
            pub flags: Cow<'a, [$flags]>,

            /// Used to indicate the hostname or IP address of the endpoint.
            pub endpoint_host: STR0_255,

            /// Used to indicate the connecting port value of the endpoint.
            pub endpoint_port: u16,

            /// The following fields relay the new_mining device information.
            ///
            /// Used to indicate the vendor/manufacturer of the device.
            pub vendor: STR0_255,

            /// Used to indicate the hardware version of the device.
            pub hardware_version: STR0_255,

            /// Used to indicate the firmware on the device.
            pub firmware: STR0_255,

            /// Used to indicate the unique identifier of the device defined by the
            /// vendor.
            pub device_id: STR0_255,
        }

        impl<'a> $conn_type<'a> {
            pub fn new<T: Into<String>>(
                min_version: u16,
                max_version: u16,
                flags: Cow<'a, [$flags]>,
                endpoint_host: T,
                endpoint_port: u16,
                vendor: T,
                hardware_version: T,
                firmware: T,
                device_id: T,
            ) -> Result<$conn_type> {
                let vendor = vendor.into();
                if *&vendor.is_empty() {
                    return Err(Error::RequirementError(
                        "vendor field in SetupConnection MUST NOT be empty".into(),
                    ));
                }

                let firmware = firmware.into();
                if *&firmware.is_empty() {
                    return Err(Error::RequirementError(
                        "firmware field in SetupConnection MUST NOT be empty".into(),
                    ));
                }

                if min_version < 2 {
                    return Err(Error::VersionError("min_version must be atleast 2".into()));
                }

                if max_version < 2 {
                    return Err(Error::VersionError("max_version must be atleast 2".into()));
                }

                Ok($conn_type {
                    protocol: $protocol,
                    min_version,
                    max_version,
                    flags,
                    endpoint_host: STR0_255::new(endpoint_host)?,
                    endpoint_port,
                    vendor: STR0_255::new(vendor)?,
                    hardware_version: STR0_255::new(hardware_version)?,
                    firmware: STR0_255::new(firmware)?,
                    device_id: STR0_255::new(device_id)?,
                })
            }
        }

        /// Implementation of the Serializable trait to serialize the contents
        /// of the SetupConnection message to the valid message format.
        impl<'a> Serializable for $conn_type<'a> {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let byte_flags = self
                    .flags
                    .iter()
                    .map(|x| x.as_bit_flag())
                    .fold(0, |accumulator, byte| (accumulator | byte))
                    .to_le_bytes();

                let buffer = serialize!(
                    &[self.protocol as u8],
                    &self.min_version.to_le_bytes(),
                    &self.max_version.to_le_bytes(),
                    &byte_flags,
                    &self.endpoint_host.as_bytes(),
                    &self.endpoint_port.to_le_bytes(),
                    &self.vendor.as_bytes(),
                    &self.hardware_version.as_bytes(),
                    &self.firmware.as_bytes(),
                    &self.device_id.as_bytes()
                );

                Ok(writer.write(&buffer)?)
            }
        }

        /// Implementation of the Framable trait to build a network frame for the
        /// SetupConnection message.
        impl<'a> Framable for $conn_type<'a> {
            fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let mut payload = Vec::new();
                let size = *&self.serialize(&mut payload)?;

                // A size_u24 of the message payload.
                let mut payload_length = (size as u16).to_le_bytes().to_vec();
                payload_length.push(0x00);

                let buffer = serialize!(
                    &[0x00, 0x00],                           // empty extension type
                    &[MessageTypes::SetupConnection.into()], // msg_type
                    &payload_length,
                    &payload
                );

                Ok(writer.write(&buffer)?)
            }
        }

        // TODO:
        // 1. Error handling
        //   - [] If the first byte doesn't match, raise an error
        //   - [] Read bytes and assign bytes to a deserialized type
        //   - [] Pass the variables into a constructor and return errors or value
        // 2. Add docstrings
        impl<'a> Deserializable for $conn_type<'a> {
            fn deserialize(bytes: &[u8]) -> Result<$conn_type<'a>> {
                // TODO: Don't handle errors yet.
                // TODO: Fuzz test this
                // let offset = 0;
                // let protocol_byte = &bytes[offset];
                // TODO: Maybe return an error if the bytes doesn't match
                // the protocol impl
                // let protocol = Protocol::from(*protocol_byte);

                let start = 1;
                let offset = 3;
                let min_version_bytes = &bytes[start..offset];
                let min_version = (min_version_bytes[1] as u16) << 8 | min_version_bytes[0] as u16;

                let start = offset;
                let offset = 5;
                let max_version_bytes = &bytes[start..offset];
                let max_version = (max_version_bytes[1] as u16) << 8 | max_version_bytes[0] as u16;

                let start = offset;
                let offset = start + 4;
                let flags_bytes = &bytes[start..offset];
                // TODO: This might apply to all conversions right? I think using the accumulator and fold
                // won't work for high numbers
                let set_flags = flags_bytes
                    .iter()
                    .map(|x| *x as u32)
                    .fold(0, |accumulator, byte| (accumulator | byte));
                let flags = Cow::from($flags::deserialize_flags(set_flags));

                let mut start = offset;
                let endpoint_host_length = *&bytes[start] as usize;
                start += 1;
                let offset = start + endpoint_host_length;
                let endpoint_host = &bytes[start..offset];

                let start = offset;
                let offset = start + 2;
                let endpoint_port_bytes = &bytes[start..offset];
                // TODO: This might apply to all conversions right? I think using the accumulator and fold
                // won't work for high numbers
                let endpoint_port =
                    (endpoint_port_bytes[1] as u16) << 8 | endpoint_port_bytes[0] as u16;

                let mut start = offset;
                let vendor_length = *&bytes[start] as u8;
                start += 1;
                let offset = start as u8 + vendor_length;
                let vendor = &bytes[start..offset as usize];

                let mut start = offset;
                let hardware_version_length = *&bytes[start as usize] as u8;
                start += 1;
                let offset = start + hardware_version_length;
                let hardware_version = &bytes[start as usize..offset as usize];

                let mut start = offset;
                let firmware_length = *&bytes[start as usize] as u8;
                start += 1;
                let offset = start + firmware_length;
                let firmware = &bytes[start as usize..offset as usize];

                let mut start = offset;
                let device_id_length = *&bytes[start as usize] as u8;
                start += 1;
                let offset = start + device_id_length;
                let device_id = &bytes[start as usize..offset as usize];

                $conn_type::new(
                    min_version,
                    max_version,
                    flags,
                    str::from_utf8(endpoint_host)?,
                    endpoint_port,
                    str::from_utf8(vendor)?,
                    str::from_utf8(hardware_version)?,
                    str::from_utf8(firmware)?,
                    str::from_utf8(device_id)?,
                )
            }
        }
    };
}

/// Implementation of the requirements for the flags in the SetupConnection
/// messages for each sub protocol.
macro_rules! impl_message_flag {
    ($flag_type:ident, $($variant:path => $shift:expr),*) => {

        impl BitFlag for $flag_type {
            /// Gets the set bit representation of a SetupConnectionFlag as a u32.
            ///
            /// # Example
            ///
            /// ```rust
            /// use stratumv2::common::BitFlag;
            /// use stratumv2::mining;
            ///
            /// let standard_job = mining::SetupConnectionFlags::RequiresStandardJobs.as_bit_flag();
            /// assert_eq!(standard_job, 0x01);
            /// ```
            fn as_bit_flag(&self) -> u32 {
                match self {
                    $($variant => (1 << $shift)),*
                }
            }

            /// Gets a vector of enums representing message flags.
            ///
            /// # Example
            ///
            /// ```rust
            /// use stratumv2::common::BitFlag;
            /// use stratumv2::mining;
            ///
            /// let flags = mining::SetupConnectionFlags::deserialize_flags(3);
            /// assert_eq!(flags[0], mining::SetupConnectionFlags::RequiresStandardJobs);
            /// assert_eq!(flags[1], mining::SetupConnectionFlags::RequiresWorkSelection);
            /// ```
            fn deserialize_flags(flags: u32) -> Vec<$flag_type> {
                let mut result = Vec::new();

                $(if flags & $variant.as_bit_flag() != 0 {
                    result.push($variant)
                })*

                result
            }
        }
    };
}

/// An internal macro for implementing the From trait for existing Error types
/// into the projects Error type variants.
macro_rules! impl_error_conversions {
    ($($error_type:path => $error_variant:path),*) => {
        $(impl From<$error_type> for Error {
            fn from(err: $error_type) -> Error {
                $error_variant(err)
            }
        })*
    };
}
