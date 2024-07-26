use crate::result::Result;
use std::path::Path;

#[derive(Debug)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    #[allow(clippy::missing_errors_doc)]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Buffer> {
        let buf = std::fs::read_to_string(path.as_ref())?;

        Ok(Buffer {
            lines: buf.lines().map(ToString::to_string).collect(),
        })
    }

    #[must_use]
    pub fn new() -> Buffer {
        Buffer { lines: Vec::new() }
    }
}
