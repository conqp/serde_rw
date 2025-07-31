use std::path::Path;

/// Extract the file extension from a path.
pub fn extension(path: &Path) -> anyhow::Result<String> {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_lowercase)
        .ok_or_else(|| anyhow::Error::msg("missing extension"))
}
