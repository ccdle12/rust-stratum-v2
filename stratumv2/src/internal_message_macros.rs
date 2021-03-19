/// Implemention of the requirements for a SetupConnection message for each
/// sub protocol.
macro_rules! impl_setup_connection {
    ($protocol:expr, $flags:ident) => {
        use std::convert::TryInto;

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

                let buffer = serialize_slices!(
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

        impl<'a> Deserializable for SetupConnection<'a> {
            fn deserialize(bytes: &[u8]) -> Result<SetupConnection<'a>> {
                let mut parser = ByteParser::new(bytes, 0);

                let protocol = parser.next_by(1)?[0];
                if Protocol::from(protocol) == Protocol::Unknown {
                    return Err(Error::DeserializationError(
                        "received unknown protocol byte in setup connection message".into(),
                    ));
                }

                let min_version = parser.next_by(2)?;
                let max_version = parser.next_by(2)?;

                let set_flags = parser
                    .next_by(4)?
                    .iter()
                    .map(|x| *x as u32)
                    .fold(0, |accumulator, byte| (accumulator | byte));

                let endpoint_host_length = parser.next_by(1)?[0] as usize;
                let endpoint_host = parser.next_by(endpoint_host_length)?;

                let endpoint_port = parser.next_by(2)?;

                let vendor_length = parser.next_by(1)?[0] as usize;
                let vendor = parser.next_by(vendor_length)?;

                let hardware_version_length = parser.next_by(1)?[0] as usize;
                let hardware_version = parser.next_by(hardware_version_length)?;

                let firmware_length = parser.next_by(1)?[0] as usize;
                let firmware = parser.next_by(firmware_length)?;

                let device_id_length = parser.next_by(1)?[0] as usize;
                let device_id = parser.next_by(device_id_length)?;

                SetupConnection::new(
                    u16::from_le_bytes(min_version.try_into()?),
                    u16::from_le_bytes(max_version.try_into()?),
                    Cow::from($flags::deserialize_flags(set_flags)),
                    str::from_utf8(endpoint_host)?,
                    u16::from_le_bytes(endpoint_port.try_into()?),
                    str::from_utf8(vendor)?,
                    str::from_utf8(hardware_version)?,
                    str::from_utf8(firmware)?,
                    str::from_utf8(device_id)?,
                )
            }
        }

        impl_frameable_trait_with_lifetime!(SetupConnection, MessageTypes::SetupConnection, false, 'a);
    };
}

macro_rules! impl_setup_connection_success {
    ($flags:ident) => {
        /// SetupConnectionSuccess is one of the required responses from a
        /// Server to a Client when a connection is accepted.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::borrow::Cow;
        /// use stratumv2::mining;
        ///
        /// let conn_success = mining::SetupConnectionSuccess::new(
        ///    2,
        ///    Cow::Borrowed(&[
        ///        mining::SetupConnectionSuccessFlags::RequiresFixedVersion,
        ///     ]),
        /// );
        /// assert_eq!(
        ///     conn_success.flags[0],
        ///     mining::SetupConnectionSuccessFlags::RequiresFixedVersion
        /// );
        /// ```
        pub struct SetupConnectionSuccess<'a> {
            /// Version proposed by the connecting node as one of the verions supported
            /// by the upstream node. The version will be used during the lifetime of
            /// the connection.
            pub used_version: u16,

            /// Indicates the optional features the server supports.
            pub flags: Cow<'a, [$flags]>,
        }

        impl<'a> SetupConnectionSuccess<'a> {
            /// Constructor for the SetupConnectionSuccess message.
            pub fn new(used_version: u16, flags: Cow<'a, [$flags]>) -> SetupConnectionSuccess {
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

                let buffer = serialize_slices!(&self.used_version.to_le_bytes(), &byte_flags);
                Ok(writer.write(&buffer)?)
            }
        }

        impl<'a> Deserializable for SetupConnectionSuccess<'a> {
            fn deserialize(bytes: &[u8]) -> Result<SetupConnectionSuccess<'a>> {
                let mut parser = ByteParser::new(bytes, 0);

                let used_version_bytes = parser.next_by(2)?;
                let set_flags = parser
                    .next_by(4)?
                    .iter()
                    .map(|x| *x as u32)
                    .fold(0, |accumulator, byte| (accumulator | byte));

                Ok(SetupConnectionSuccess {
                    used_version: u16::from_le_bytes(used_version_bytes.try_into()?),
                    flags: Cow::from($flags::deserialize_flags(set_flags)),
                })
            }
        }

        impl_frameable_trait_with_lifetime!(SetupConnectionSuccess, MessageTypes::SetupConnectionSuccess, false, 'a);
    };
}

/// Implementation of the SetupConnectionError message for each sub protocol.
macro_rules! impl_setup_connection_error {
    ($flag_type:ident) => {
        /// SetupConnectionError is one of the required responses from a Server
        /// to a Client when a new connection has failed. The server is required
        /// to send this message with an error code before closing the connection.
        ///
        /// If the error is a variant of [UnsupportedFeatureFlags](enum.SetupConnectionErrorCodes.html),
        /// the server MUST respond with all the feature flags that it does NOT support.
        ///
        /// If the flag is 0, then the error is some condition aside from unsupported
        /// flags.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::borrow::Cow;
        /// use stratumv2::mining;
        /// use stratumv2::common::SetupConnectionErrorCodes;
        ///
        /// let conn_error = mining::SetupConnectionError::new(
        ///    Cow::Borrowed(&[
        ///        mining::SetupConnectionFlags::RequiresVersionRolling,
        ///     ]),
        ///        SetupConnectionErrorCodes::UnsupportedFeatureFlags
        /// );
        ///
        /// assert!(conn_error.is_ok());
        /// assert_eq!(
        ///     conn_error.unwrap().error_code,
        ///     SetupConnectionErrorCodes::UnsupportedFeatureFlags
        /// );
        /// ```
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

                let result = serialize_slices!(
                    &byte_flags,
                    &STR0_255::new(&self.error_code.to_string())?.as_bytes()
                );

                Ok(writer.write(&result)?)
            }
        }

        impl<'a> Deserializable for SetupConnectionError<'a> {
            fn deserialize(bytes: &[u8]) -> Result<SetupConnectionError<'a>> {
                let mut parser = ByteParser::new(bytes, 0);

                let set_flags = parser
                    .next_by(4)?
                    .iter()
                    .map(|x| *x as u32)
                    .fold(0, |accumulator, byte| (accumulator | byte));

                let error_code_length = parser.next_by(1)?[0] as usize;
                let error_code = str::from_utf8(parser.next_by(error_code_length)?)?;

                Ok(SetupConnectionError {
                    flags: Cow::from($flag_type::deserialize_flags(set_flags)),
                    error_code: SetupConnectionErrorCodes::from_str(error_code)?,
                })
            }
        }

        impl_frameable_trait_with_lifetime!(SetupConnectionError, MessageTypes::SetupConnectionError, false, 'a);
    };
}

/// Implementation of the OpenMiningChannelError. This message applies to both
/// Standard Mining Channels and Extended Mining Channels.
macro_rules! impl_open_mining_channel_error {
    ($name:ident, $msg_type:path) => {
        pub struct $name {
            request_id: u32,
            error_code: OpenMiningChannelErrorCodes,
        }

        impl $name {
            pub fn new(request_id: u32, error_code: OpenMiningChannelErrorCodes) -> $name {
                $name {
                    request_id,
                    error_code,
                }
            }
        }

        impl Serializable for $name {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let buffer = serialize_slices!(
                    &self.request_id.to_le_bytes(),
                    &STR0_32::new(self.error_code.to_string())?.as_bytes()
                );

                Ok(writer.write(&buffer)?)
            }
        }

        impl Deserializable for $name {
            fn deserialize(bytes: &[u8]) -> Result<$name> {
                let mut parser = ByteParser::new(bytes, 0);

                let request_id = parser.next_by(4)?;
                let error_code_length = parser.next_by(1)?[0] as usize;
                let error_code = str::from_utf8(parser.next_by(error_code_length)?)?;

                Ok($name::new(
                    u32::from_le_bytes(request_id.try_into()?),
                    OpenMiningChannelErrorCodes::from_str(error_code)?,
                ))
            }
        }

        impl_frameable_trait!($name, $msg_type, false);
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

/// Implemenation of all the common traits for ErrorCode enums.
macro_rules! impl_error_codes_enum {
    ($name:ident, $($variant:path => $str:expr),*) => {
        use std::str::FromStr;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    $($variant => write!(f, $str)),*
                }
            }
        }


        impl FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self> {
                match s {
                    $($str => Ok($variant)),*,
                    _ => Err(Error::UnknownErrorCode()),
                }
            }
        }
    };
}

/// An internal macro to implement the Frameable trait for messages. Some mesages
/// require the extenstion type to have a channel_msg bit set since the message
/// is intended for a specific channel_id. The channel_id will always be found
/// in the deserialized object as a field.
macro_rules! impl_frameable_trait {
    ($msg:ident, $msg_type:path, $has_channel_msg_bit:expr) => {
        impl Frameable for $msg {
            internal_frameable_trait!($msg_type, $has_channel_msg_bit);
        }
    };
}

macro_rules! impl_frameable_trait_with_lifetime {
    ($msg:ident, $msg_type:path, $has_channel_msg_bit:expr, $lt:lifetime) => {
        impl<$lt> Frameable for $msg<$lt> {
            internal_frameable_trait!($msg_type, $has_channel_msg_bit);
        }
    };
}

// TODO: Implement a conditional branch to set the channel msg bit.
macro_rules! internal_frameable_trait {
    ($msg_type:path, $has_channel_msg_bit:expr) => {
        fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
            let mut payload = Vec::new();
            let size = *&self.serialize(&mut payload)?;

            // A size_u24 of the message payload.
            let payload_length = (size as u32).to_le_bytes()[0..=2].to_vec();

            let buffer = serialize_slices!(
                &[0x00, 0x00],       // empty extension type
                &[$msg_type.into()], // msg_type
                &payload_length,
                &payload
            );

            Ok(writer.write(&buffer)?)
        }
    };
}

// CCDLE12 TMP:
// TODO: Figure out how to reuse fields between Standard and Extended Mining Channels,
// while keeping the doc comments.
macro_rules! impl_open_standard_mining_channel {
    () => {
        /// OpenStandardMiningChannel is a message sent by the Client to the Server
        /// after a [SetupConnection.Success](struct.SetupConnectionSuccess.html) is
        /// sent from the Server. This message is used to request opening a standard
        /// channel to the upstream server. A standard mining channel indicates `header-only`
        /// mining.
        pub struct OpenStandardMiningChannel {
            /// A Client-specified unique identifier across all client connections.
            /// The request_id is not interpreted by the Server.
            pub request_id: u32,

            /// A sequence of bytes that identifies the node to the Server, e.g.
            /// "braiintest.worker1".
            pub user_identity: STR0_255,

            /// The expected [h/s] (hash rate/per second) of the
            /// device or the cumulative on the channel if multiple devices are connected
            /// downstream. Proxies MUST send 0.0f when there are no mining devices
            /// connected yet.
            pub nominal_hash_rate: f32,

            /// The Maximum Target that can be acceptd by the connected device or
            /// multiple devices downstream. The Server MUST accept the maximum
            /// target or respond by sending a
            /// [OpenStandardMiningChannel.Error](struct.OpenStandardMiningChannelError.html)
            /// or [OpenExtendedMiningChannel.Error](struct.OpenExtendedMiningChannelError.html)
            pub max_target: U256,
        }

        impl OpenStandardMiningChannel {
            pub fn new<T: Into<String>>(
                request_id: u32,
                user_identity: T,
                nominal_hash_rate: f32,
                max_target: U256,
            ) -> Result<OpenStandardMiningChannel> {
                Ok(OpenStandardMiningChannel {
                    request_id,
                    user_identity: STR0_255::new(user_identity)?,
                    nominal_hash_rate,
                    max_target,
                })
            }
        }

        impl Serializable for OpenStandardMiningChannel {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let buffer = serialize_slices!(
                    &self.request_id.to_le_bytes(),
                    &self.user_identity.as_bytes(),
                    &self.nominal_hash_rate.to_le_bytes(),
                    &self.max_target
                );

                Ok(writer.write(&buffer)?)
            }
        }

        impl Deserializable for OpenStandardMiningChannel {
            fn deserialize(bytes: &[u8]) -> Result<OpenStandardMiningChannel> {
                let mut parser = ByteParser::new(bytes, 0);

                let request_id = parser.next_by(4)?;
                let user_identity_length = parser.next_by(1)?[0] as usize;
                let user_identity = parser.next_by(user_identity_length)?;
                let nominal_hash_rate = parser.next_by(4)?;
                let max_target = parser.next_by(32)?;

                OpenStandardMiningChannel::new(
                    u32::from_le_bytes(request_id.try_into()?),
                    str::from_utf8(user_identity)?,
                    f32::from_le_bytes(nominal_hash_rate.try_into()?),
                    max_target.try_into()?,
                )
            }
        }

        impl_frameable_trait!(
            OpenStandardMiningChannel,
            MessageTypes::OpenStandardMiningChannel,
            false
        );
    };
}

macro_rules! impl_open_extended_mining_channel {
    () => {
        /// OpenExtendedMiningChannel is a message sent by the Client to the Server
        /// to open a mining channe that has additional capabilities such as
        /// difficulty aggregate and custom search space splitting.
        pub struct OpenExtendedMiningChannel {
            /// A Client-specified unique identifier across all client connections.
            /// The request_id is not interpreted by the Server.
            pub request_id: u32,

            /// A sequence of bytes that identifies the node to the Server, e.g.
            /// "braiintest.worker1".
            pub user_identity: STR0_255,

            /// The expected [h/s] (hash rate/per second) of the
            /// device or the cumulative on the channel if multiple devices are connected
            /// downstream. Proxies MUST send 0.0f when there are no mining devices
            /// connected yet.
            pub nominal_hash_rate: f32,

            /// The Maximum Target that can be acceptd by the connected device or
            /// multiple devices downstream. The Server MUST accept the maximum
            /// target or respond by sending a
            /// [OpenStandardMiningChannel.Error](struct.OpenStandardMiningChannelError.html)
            /// or [OpenExtendedMiningChannel.Error](struct.OpenExtendedMiningChannelError.html)
            pub max_target: U256,

            /// The minimum size of extranonce space required by the Downstream node.
            pub min_extranonce_size: u16,
        }

        impl OpenExtendedMiningChannel {
            pub fn new<T: Into<String>>(
                request_id: u32,
                user_identity: T,
                nominal_hash_rate: f32,
                max_target: U256,
                min_extranonce_size: u16,
            ) -> Result<OpenExtendedMiningChannel> {
                Ok(OpenExtendedMiningChannel {
                    request_id,
                    user_identity: STR0_255::new(user_identity)?,
                    nominal_hash_rate,
                    max_target,
                    min_extranonce_size,
                })
            }
        }

        impl Serializable for OpenExtendedMiningChannel {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let buffer = serialize_slices!(
                    &self.request_id.to_le_bytes(),
                    &self.user_identity.as_bytes(),
                    &self.nominal_hash_rate.to_le_bytes(),
                    &self.max_target,
                    &self.min_extranonce_size.to_le_bytes()
                );

                Ok(writer.write(&buffer)?)
            }
        }

        impl Deserializable for OpenExtendedMiningChannel {
            fn deserialize(bytes: &[u8]) -> Result<OpenExtendedMiningChannel> {
                let mut parser = ByteParser::new(bytes, 0);

                let request_id = parser.next_by(4)?;
                let user_identity_length = parser.next_by(1)?[0] as usize;
                let user_identity = parser.next_by(user_identity_length)?;
                let nominal_hash_rate = parser.next_by(4)?;
                let max_target = parser.next_by(32)?;
                let min_extranonce_size = parser.next_by(2)?;

                OpenExtendedMiningChannel::new(
                    u32::from_le_bytes(request_id.try_into()?),
                    str::from_utf8(user_identity)?,
                    f32::from_le_bytes(nominal_hash_rate.try_into()?),
                    max_target.try_into()?,
                    u16::from_le_bytes(min_extranonce_size.try_into()?),
                )
            }
        }

        impl_frameable_trait!(
            OpenExtendedMiningChannel,
            MessageTypes::OpenExtendedMiningChannel,
            false
        );
    };
}

// CCDLE12 TMP:
// TODO: This is also in a temp state, this could be moved back to the mining mod
// for readability until this is fixed. Figure out how to reuse fields between
// Standard and Extended Mining Channel messages, while keeping the doc comments.
macro_rules! impl_open_standard_mining_channel_success {
    () => {
        /// OpenStandardMiningChannelSuccess is a message sent by the Server to the Client
        /// in response to a successful opening of a standard mining channel.
        pub struct OpenStandardMiningChannelSuccess {
            /// The request_id received in the
            /// [OpenStandardMiningChannel](struct.OpenStandardMiningChannel.html) message.
            /// This is returned to the Client so that they can pair the responses with the
            /// initial request.
            request_id: u32,

            /// Assigned by the Server to uniquely identify the channel, the id is stable
            /// for the whole lifetime of the connection.
            channel_id: u32,

            /// The initial target difficulty target for the mining channel.
            target: U256,

            // TODO: I don't understand the purpose of the extranonce_prefix.
            extranonce_prefix: B0_32,

            /// Group channel that the channel belongs to.
            group_channel_id: u32,
        }

        impl OpenStandardMiningChannelSuccess {
            pub fn new<T: Into<Vec<u8>>>(
                request_id: u32,
                channel_id: u32,
                target: U256,
                extranonce_prefix: T,
                group_channel_id: u32,
            ) -> Result<OpenStandardMiningChannelSuccess> {
                Ok(OpenStandardMiningChannelSuccess {
                    request_id,
                    channel_id,
                    target,
                    extranonce_prefix: B0_32::new(extranonce_prefix.into())?,
                    group_channel_id,
                })
            }
        }

        impl Serializable for OpenStandardMiningChannelSuccess {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let buffer = serialize_slices!(
                    &self.request_id.to_le_bytes(),
                    &self.channel_id.to_le_bytes(),
                    &self.target,
                    &self.extranonce_prefix.as_bytes(),
                    &self.group_channel_id.to_le_bytes()
                );

                Ok(writer.write(&buffer)?)
            }
        }

        impl Deserializable for OpenStandardMiningChannelSuccess {
            fn deserialize(bytes: &[u8]) -> Result<OpenStandardMiningChannelSuccess> {
                let mut parser = ByteParser::new(bytes, 0);

                let request_id = parser.next_by(4)?;
                let channel_id = parser.next_by(4)?;
                let target = parser.next_by(32)?;
                let extranonce_prefix_length = parser.next_by(1)?[0] as usize;
                let extranonce_prefix = parser.next_by(extranonce_prefix_length)?;
                let group_channel_id = parser.next_by(4)?;

                OpenStandardMiningChannelSuccess::new(
                    u32::from_le_bytes(request_id.try_into()?),
                    u32::from_le_bytes(channel_id.try_into()?),
                    target.try_into()?,
                    extranonce_prefix.to_vec(),
                    u32::from_le_bytes(group_channel_id.try_into()?),
                )
            }
        }

        impl_frameable_trait!(
            OpenStandardMiningChannelSuccess,
            MessageTypes::OpenStandardMiningChannelSuccess,
            false
        );
    };
}

macro_rules! impl_open_extended_mining_channel_success {
    () => {
        /// OpenExtendedMiningChannelSuccess is a message sent by the Server to the Client
        /// in response to a successful opening of a standard mining channel.
        pub struct OpenExtendedMiningChannelSuccess {
            /// The request_id received in the
            /// [OpenExtendedMiningChannel](struct.OpenExtendedMiningChannel.html) message.
            /// This is returned to the Client so that they can pair the responses with the
            /// initial request.
            request_id: u32,

            /// Assigned by the Server to uniquely identify the channel, the id is stable
            /// for the whole lifetime of the connection.
            channel_id: u32,

            /// The initial target difficulty target for the mining channel.
            target: U256,

            // TODO: I don't understand the purpose of the extranonce size.
            extranonce_size: u16,

            // TODO: I don't understand the purpose of the extranonce prefix.
            extranonce_prefix: B0_32,
        }

        impl OpenExtendedMiningChannelSuccess {
            pub fn new<T: Into<Vec<u8>>>(
                request_id: u32,
                channel_id: u32,
                target: U256,
                extranonce_size: u16,
                extranonce_prefix: T,
            ) -> Result<OpenExtendedMiningChannelSuccess> {
                Ok(OpenExtendedMiningChannelSuccess {
                    request_id,
                    channel_id,
                    target,
                    extranonce_size,
                    extranonce_prefix: B0_32::new(extranonce_prefix.into())?,
                })
            }
        }

        impl Serializable for OpenExtendedMiningChannelSuccess {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                let buffer = serialize_slices!(
                    &self.request_id.to_le_bytes(),
                    &self.channel_id.to_le_bytes(),
                    &self.target,
                    &self.extranonce_size.to_le_bytes(),
                    &self.extranonce_prefix.as_bytes()
                );

                Ok(writer.write(&buffer)?)
            }
        }

        impl Deserializable for OpenExtendedMiningChannelSuccess {
            fn deserialize(bytes: &[u8]) -> Result<OpenExtendedMiningChannelSuccess> {
                let mut parser = ByteParser::new(bytes, 0);

                let request_id = parser.next_by(4)?;
                let channel_id = parser.next_by(4)?;
                let target = parser.next_by(32)?;
                let extranonce_size = parser.next_by(2)?;
                let extranonce_prefix_length = parser.next_by(1)?[0] as usize;
                let extranonce_prefix = parser.next_by(extranonce_prefix_length)?;

                OpenExtendedMiningChannelSuccess::new(
                    u32::from_le_bytes(request_id.try_into()?),
                    u32::from_le_bytes(channel_id.try_into()?),
                    target.try_into()?,
                    u16::from_le_bytes(extranonce_size.try_into()?),
                    extranonce_prefix.to_vec(),
                )
            }
        }

        impl_frameable_trait!(
            OpenExtendedMiningChannelSuccess,
            MessageTypes::OpenExtendedMiningChannelSuccess,
            false
        );
    };
}
