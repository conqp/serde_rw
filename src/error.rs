use std::fmt::{Debug, Display, Formatter};
use std::io::IntoInnerError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    InvalidExtension(Option<String>),
    SerdeError(Box<dyn std::error::Error>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => <std::io::Error as Display>::fmt(error, f),
            Self::InvalidExtension(string) => write!(
                f,
                r#"Invalid extension: "{}""#,
                string.as_deref().unwrap_or("")
            ),
            Self::SerdeError(error) => Display::fmt(error, f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self::SerdeError(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Self::SerdeError(error.into())
    }
}

impl<W> From<IntoInnerError<W>> for Error {
    fn from(error: IntoInnerError<W>) -> Self {
        Self::from(error.into_error())
    }
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error.into())
    }
}

#[cfg(feature = "toml")]
impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::SerdeError(error.into())
    }
}

#[cfg(feature = "toml")]
impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        Self::SerdeError(error.into())
    }
}

#[cfg(feature = "xml")]
impl From<quick_xml::DeError> for Error {
    fn from(error: quick_xml::DeError) -> Self {
        Self::SerdeError(error.into())
    }
}

#[cfg(feature = "yaml")]
impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Self::SerdeError(error.into())
    }
}
