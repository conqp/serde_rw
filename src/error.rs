use serde::{de, ser};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    InvalidExtension(Option<String>),
    SerializationError(Box<dyn ser::StdError>),
    DeserializationError(Box<dyn de::StdError>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(error) => write!(f, "{error}"),
            Self::InvalidExtension(string) => write!(
                f,
                r#"Invalid extension: "{}""#,
                string.as_deref().unwrap_or("")
            ),
            Self::SerializationError(error) | Self::DeserializationError(error) => {
                write!(f, "{error}")
            }
        }
    }
}
