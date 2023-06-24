use crate::Error;
use std::path::Path;

pub fn extension(path: &Path) -> Result<&str, Error> {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(Error::InvalidExtension(None))
}
