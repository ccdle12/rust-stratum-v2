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
    ($protocol:expr, $flags:ident) => {
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
        pub struct SetupConnection<'a> {
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

        impl<'a> SetupConnection<'a> {
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
            ) -> Result<SetupConnection> {
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

                Ok(SetupConnection {
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
        impl<'a> Serializable for SetupConnection<'a> {
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
        impl<'a> Framable for SetupConnection<'a> {
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

        impl<'a> Deserializable for SetupConnection<'a> {
            fn deserialize(bytes: &[u8]) -> Result<SetupConnection<'a>> {
                // TODO: Fuzz test this
                let offset = 0;
                let protocol_byte = bytes.get(offset);
                if protocol_byte.is_none() {
                    return Err(Error::DeserializationError(
                        "received empty bytes in setup connection message".into(),
                    ));
                }

                if Protocol::from(*protocol_byte.unwrap()) == Protocol::Unknown {
                    return Err(Error::DeserializationError(
                        "received unknown protocol byte in setup connection message".into(),
                    ));
                }

                // Get the min_version bytes.
                let start = 1;
                let offset = 3;
                let min_version_bytes = &bytes.get(start..offset);
                if min_version_bytes.is_none() {
                    return Err(Error::DeserializationError(
                        "min_version is missing from setup connection message".into(),
                    ));
                }
                let min_version = (min_version_bytes.unwrap()[1] as u16) << 8
                    | min_version_bytes.unwrap()[0] as u16;

                // Get the max_version bytes.
                let start = offset;
                let offset = 5;
                let max_version_bytes = &bytes.get(start..offset);
                if max_version_bytes.is_none() {
                    return Err(Error::DeserializationError(
                        "max_version is missing from setup connection message".into(),
                    ));
                }
                let max_version = (max_version_bytes.unwrap()[1] as u16) << 8
                    | max_version_bytes.unwrap()[0] as u16;

                // Get the flag bytes.
                let start = offset;
                let offset = start + 4;
                let flags_bytes = &bytes.get(start..offset);
                if flags_bytes.is_none() {
                    return Err(Error::DeserializationError(
                        "setup connection flags are missing from setup connection message".into(),
                    ));
                }
                let set_flags = flags_bytes
                    .unwrap()
                    .iter()
                    .map(|x| *x as u32)
                    .fold(0, |accumulator, byte| (accumulator | byte));
                let flags = Cow::from($flags::deserialize_flags(set_flags));

                // Get the endpoint_host_length.
                let mut start = offset;
                let endpoint_host_length = &bytes.get(start);
                if endpoint_host_length.is_none() {
                    return Err(Error::DeserializationError(
                        "endpoint_host length is missing from setup connection message".into(),
                    ));
                }

                // Get the endpoint_host.
                start += 1;
                let offset = start + *endpoint_host_length.unwrap() as usize;
                let endpoint_host = &bytes.get(start..offset);
                if endpoint_host.is_none() {
                    return Err(Error::DeserializationError(
                        "endpoint_host is missing from setup connection message".into(),
                    ));
                }

                // Get the variable length bytes for the endpoint host.
                let start = offset;
                let offset = start + 2;
                let endpoint_port_bytes = &bytes.get(start..offset);
                if endpoint_port_bytes.is_none() {
                    return Err(Error::DeserializationError(
                        "endpoint_port is missing from setup connection message".into(),
                    ));
                }
                let endpoint_port = (endpoint_port_bytes.unwrap()[1] as u16) << 8
                    | endpoint_port_bytes.unwrap()[0] as u16;

                // Get the vendor bytes length.
                let mut start = offset;
                let vendor_length = &bytes.get(start);
                if vendor_length.is_none() {
                    return Err(Error::DeserializationError(
                        "vendor is missing from setup connection message".into(),
                    ));
                }

                // Get the vendor bytes.
                start += 1;
                let offset = start as u8 + vendor_length.unwrap();
                let vendor = &bytes.get(start..offset as usize);
                if vendor.is_none() {
                    return Err(Error::DeserializationError(
                        "vendor is missing from setup connection message".into(),
                    ));
                }

                // Get the hardware version length.
                let mut start = offset;
                let hardware_version_length = bytes.get(start as usize);
                if hardware_version_length.is_none() {
                    return Err(Error::DeserializationError(
                        "hardware version length is missing from setup connection message".into(),
                    ));
                }
                start += 1;
                let offset = start + hardware_version_length.unwrap();
                let hardware_version = &bytes.get(start as usize..offset as usize);
                if hardware_version.is_none() {
                    return Err(Error::DeserializationError(
                        "hardware version is missing from setup connection message".into(),
                    ));
                }

                // Get the firmware length.
                let mut start = offset;
                let firmware_length = &bytes.get(start as usize);
                if firmware_length.is_none() {
                    return Err(Error::DeserializationError(
                        "firmware length is missing from setup connection message".into(),
                    ));
                }

                // Get the firmware.
                start += 1;
                let offset = start + firmware_length.unwrap();
                let firmware = &bytes.get(start as usize..offset as usize);
                if firmware.is_none() {
                    return Err(Error::DeserializationError(
                        "firmware is missing from setup connection message".into(),
                    ));
                }

                // Get device id length.
                let mut start = offset;
                let device_id_length = &bytes.get(start as usize);
                if device_id_length.is_none() {
                    return Err(Error::DeserializationError(
                        "device id length is missing from setup connection message".into(),
                    ));
                }
                start += 1;
                let offset = start + device_id_length.unwrap();

                // Get device id.
                let device_id = &bytes.get(start as usize..offset as usize);
                if device_id.is_none() {
                    return Err(Error::DeserializationError(
                        "device id is missing from setup connection message".into(),
                    ));
                }

                SetupConnection::new(
                    min_version,
                    max_version,
                    flags,
                    str::from_utf8(endpoint_host.unwrap())?,
                    endpoint_port,
                    str::from_utf8(vendor.unwrap())?,
                    str::from_utf8(hardware_version.unwrap())?,
                    str::from_utf8(firmware.unwrap())?,
                    str::from_utf8(device_id.unwrap())?,
                )
            }
        }
    };
}

macro_rules! impl_setup_connection_success {
    ($flags:ident) => {
        /// SetupConnectionSuccess is one of the required responses from a
        /// Server to a Client when a connection is accepted.
        pub struct SetupConnectionSuccess<'a> {
            /// Version proposed by the connecting node as one of the verions supported
            /// by the upstream node. The version will be used during the lifetime of
            /// the connection.
            pub used_version: u16,

            /// Indicates the optional features the server supports.
            pub flags: &'a [$flags],
        }

        impl<'a> SetupConnectionSuccess<'a> {
            /// Constructor for the SetupConnectionSuccess message.
            pub fn new(used_version: u16, flags: &[$flags]) -> SetupConnectionSuccess {
                SetupConnectionSuccess {
                    used_version,
                    flags,
                }
            }
        }

        impl Serializable for SetupConnectionSuccess<'_> {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let byte_flags = self
                    .flags
                    .iter()
                    .map(|x| x.as_bit_flag())
                    .fold(0, |accumulator, byte| (accumulator | byte))
                    .to_le_bytes();

                let buffer = serialize!(&self.used_version.to_le_bytes(), &byte_flags);
                Ok(writer.write(&buffer)?)
            }
        }

        impl Framable for SetupConnectionSuccess<'_> {
            fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let mut payload = Vec::new();
                let size = *&self.serialize(&mut payload)?;

                // A size_u24 of the message payload.
                let mut payload_length = (size as u16).to_le_bytes().to_vec();
                payload_length.push(0x00);

                let result = serialize!(
                    &[0x00, 0x00],                                  // extention_type
                    &[MessageTypes::SetupConnectionSuccess.into()], // msg_type
                    &payload_length,
                    &payload
                );

                Ok(writer.write(&result)?)
            }
        }
    };
}

/// Implementation of the SetupConnectionError message for each sub protocol.
macro_rules! impl_setup_connection_error {
    ($flag_type:ident) => {
        /// SetupConnectionError is one of the required responses from a server to client
        /// when a new connection has failed. The server is required to send this message
        /// with an error code before closing the connection.
        ///
        /// If the error is a variant of [UnsupportedFeatureFlags](enum.SetupConnectionErrorCodes.html),
        /// the server MUST respond with all the feature flags that it does NOT support.
        ///
        /// If the flag is 0, then the error is some condition aside from unsupported
        /// flags.
        pub struct SetupConnectionError<'a> {
            /// Indicates all the flags that the server does NOT support.
            pub flags: Cow<'a, [$flag_type]>,

            /// Error code is a predefined STR0_255 error code.
            pub error_code: SetupConnectionErrorCodes,
        }

        impl<'a> SetupConnectionError<'a> {
            /// Constructor for the SetupConnectionError message.
            pub fn new(
                flags: Cow<'a, [$flag_type]>,
                error_code: SetupConnectionErrorCodes,
            ) -> Result<SetupConnectionError> {
                if flags.is_empty()
                    && error_code == SetupConnectionErrorCodes::UnsupportedFeatureFlags
                {
                    return Err(Error::RequirementError(
                        "a full set of unsupported flags MUST be returned to the client".into(),
                    ));
                }

                Ok(SetupConnectionError { flags, error_code })
            }
        }

        impl Serializable for SetupConnectionError<'_> {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let byte_flags = self
                    .flags
                    .iter()
                    .map(|x| x.as_bit_flag())
                    .fold(0, |accumulator, byte| (accumulator | byte))
                    .to_le_bytes();

                let result = serialize!(
                    &byte_flags,
                    &STR0_255::new(&self.error_code.to_string())?.as_bytes()
                );

                Ok(writer.write(&result)?)
            }
        }

        impl Framable for SetupConnectionError<'_> {
            fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let mut payload = Vec::new();
                let size = *&self.serialize(&mut payload)?;

                // A size_u24 of the message payload.
                let mut payload_length = (size as u16).to_le_bytes().to_vec();
                payload_length.push(0x00);

                let result = serialize!(
                    &[0x00, 0x00], // extension_type
                    &[MessageTypes::SetupConnectionError.into()],
                    &payload_length,
                    &payload
                );

                Ok(writer.write(&result)?)
            }
        }

        impl<'a> Deserializable for SetupConnectionError<'a> {
            fn deserialize(bytes: &[u8]) -> Result<SetupConnectionError<'a>> {
                // Get the flags.
                let start = 0;
                let offset = start + 4;
                let flags_bytes = &bytes.get(start..offset);
                if flags_bytes.is_none() {
                    return Err(Error::DeserializationError(
                        "flags are missing from setup connection error message".into(),
                    ));
                }
                let set_flags = flags_bytes
                    .unwrap()
                    .iter()
                    .map(|x| *x as u32)
                    .fold(0, |accumulator, byte| (accumulator | byte));
                let flags = Cow::from($flag_type::deserialize_flags(set_flags));

                // Get the length of the error code.
                let mut start = offset;
                let error_code_length = &bytes.get(start as usize);
                if error_code_length.is_none() {
                    return Err(Error::DeserializationError(
                        "length of error code is missing from setup connection error message"
                            .into(),
                    ));
                }

                // Get the error code message.
                start += 1;
                let offset = start + *error_code_length.unwrap() as usize;
                let error_code = &bytes.get(start as usize..offset as usize);
                if error_code.is_none() {
                    return Err(Error::DeserializationError(
                        "error code is missing from setup connection message".into(),
                    ));
                }

                Ok(SetupConnectionError {
                    flags,
                    error_code: SetupConnectionErrorCodes::from(str::from_utf8(
                        error_code.unwrap(),
                    )?),
                })
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
            /// use stratumv2::BitFlag;
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
            /// use stratumv2::BitFlag;
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
