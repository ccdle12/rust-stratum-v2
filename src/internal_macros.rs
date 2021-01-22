/// A convenience macro for serializing a variable length of byte slices to a
/// Vector.
macro_rules! serialize {
    ($($x:expr),*) => {{
        let mut buffer: Vec<u8> = Vec::new();
        $( buffer.extend_from_slice($x);)*
        buffer
    }};
}

// TODO: Docstring
macro_rules! impl_setup_connection {
    ($protocol:expr, $flags:ident, $conn_type:ident) => {
        impl $conn_type {
            pub fn new<T: Into<String>>(
                min_version: u16,
                max_version: u16,
                flags: &[$flags],
                endpoint_host: T,
                endpoint_port: u16,
                vendor: T,
                hardware_version: T,
                firmware: T,
                device_id: T,
            ) -> Result<$conn_type> {
                let flags = flags
                    .iter()
                    .map(|x| x.as_bit_flag())
                    .fold(0, |acc, byte| (acc | byte));

                let internal = RawSetupConnection::new(
                    $protocol,
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

                Ok($conn_type { internal })
            }

            fn min_version(&self) -> u16 {
                self.internal.min_version
            }

            fn max_version(&self) -> u16 {
                self.internal.max_version
            }

            fn flags(&self) -> Vec<$flags> {
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
        impl Serializable for $conn_type {
            fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                Ok(writer.write(&self.internal.serialize())?)
            }
        }

        // TODO: Docstring
        impl Deserializable for $conn_type {
            fn deserialize(bytes: &[u8]) -> Result<$conn_type> {
                Ok($conn_type {
                    internal: RawSetupConnection::deserialize(bytes)?,
                })
            }
        }

        /// Implementation of the Framable trait to build a network frame for the
        /// SetupConnection message.
        impl Framable for $conn_type {
            fn frame<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
                Ok(writer.write(&self.internal.frame())?)
            }
        }
    };
}
