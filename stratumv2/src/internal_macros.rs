/// A convenience macro for serializing a variable length of byte slices to a
/// Vector.
macro_rules! serialize_slices {
    ($($x:expr),*) => {{
        let mut buffer: Vec<u8> = Vec::new();
        $( buffer.extend_from_slice($x);)*
        buffer
    }};
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

/// An internal helper macro for getting the unix time now as a u32.
macro_rules! unix_u32_now {
    () => {
        system_unix_time_to_u32(&SystemTime::now())
    };
}
