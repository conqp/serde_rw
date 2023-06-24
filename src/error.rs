use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    FileError(std::io::Error),
    InvalidExtension(Option<String>),
    SerdeError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileError(error) => write!(f, "{error}"),
            Self::InvalidExtension(string) => write!(
                f,
                r#"Invalid extension: "{}""#,
                string.as_deref().unwrap_or("")
            ),
            Self::SerdeError(message) => write!(f, "{message}"),
        }
    }
}
