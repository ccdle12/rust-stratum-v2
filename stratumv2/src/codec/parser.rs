use crate::error::{Error, Result};

/// A custom iterator-like struct. It's used to extract segments
/// from a slice using by providing an offset to return the bytes from start
/// to step.
pub struct ByteParser<'a> {
    bytes: &'a [u8],
    start: usize,
}

impl<'a> ByteParser<'a> {
    pub fn new(bytes: &'a [u8], start: usize) -> ByteParser {
        ByteParser { bytes, start }
    }

    pub fn next_by(&mut self, step: usize) -> Result<&'a [u8]> {
        let offset = self.start + step;

        let b = self.bytes.get(self.start..offset);
        if b.is_none() {
            return Err(Error::ParseError("out of bounds error".into()));
        }

        self.start = offset;
        Ok(b.unwrap())
    }
}
