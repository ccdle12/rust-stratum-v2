/// A convenience macro for serializing a variable length of bytes and byte
/// slices to a Vector.
macro_rules! serialize {
    ($($x:expr),*) => {{
        let mut buffer: Vec<u8> = Vec::new();
        $( buffer.extend_from_slice($x);)*
        buffer
    }};
}
