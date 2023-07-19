use std::path::Path;

pub fn extension(path: &Path) -> anyhow::Result<&str> {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| anyhow::Error::msg("missing extension"))
}
