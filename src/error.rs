use std::ffi::OsString;
use std::fmt;
use std::fmt::{Display, Formatter};

/// Error type for de-/serialization operations from/to files.
#[derive(Debug)]
pub enum Error {
    /// An I/O error occurred while reading from or writing to the file.
    Io(std::io::Error),
    /// Serialization or deserialization failed.
    Serde(Box<dyn std::error::Error>),
    /// The provided file extension does not indicate a supported file format.
    UnsupportedFileExtension(OsString),
    /// No file extension was specified.
    NoFileExtensionsSpecified,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::Serde(e) => e.fmt(f),
            Self::UnsupportedFileExtension(extension) => {
                write!(f, "Unsupported file extension: {}", extension.display())
            }
            Self::NoFileExtensionsSpecified => {
                write!(f, "No file extension specified.")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Serde(err) => Some(err.as_ref()),
            Self::UnsupportedFileExtension(_) | Self::NoFileExtensionsSpecified => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(Box::new(err))
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Serde(Box::new(err))
    }
}

#[cfg(feature = "toml")]
impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serde(Box::new(err))
    }
}

#[cfg(feature = "xml")]
impl From<quick_xml::DeError> for Error {
    fn from(err: quick_xml::DeError) -> Self {
        Self::Serde(Box::new(err))
    }
}
#[cfg(feature = "xml")]
impl From<quick_xml::SeError> for Error {
    fn from(err: quick_xml::SeError) -> Self {
        Self::Serde(Box::new(err))
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Serde(Box::new(err))
    }
}
