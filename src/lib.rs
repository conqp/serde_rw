use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs::{read_to_string, write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    FileError(std::io::Error),
    InvalidExtension(Option<String>),
    SerdeError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileError(error) => write!(f, "{}", error),
            Self::InvalidExtension(string) => write!(
                f,
                r#"Invalid extension: "{}""#,
                string.as_deref().unwrap_or("")
            ),
            Self::SerdeError(message) => write!(f, "{}", message),
        }
    }
}

pub trait FromFile
where
    for<'de> Self: Deserialize<'de>,
{
    fn from_file(filename: &str) -> Result<Self, Error> {
        match extension(&PathBuf::from(filename))? {
            #[cfg(feature = "json")]
            "json" => <Self as FromFile>::from_json_file(filename),
            #[cfg(feature = "toml")]
            "toml" => <Self as FromFile>::from_toml_file(filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as FromFile>::from_xml_file(filename),
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => <Self as FromFile>::from_yaml_file(filename),
            extension => Err(Error::InvalidExtension(Some(extension.to_string()))),
        }
    }

    #[cfg(feature = "json")]
    fn from_json_file(filename: &str) -> Result<Self, Error> {
        <Self as FromFile>::read_file(filename)
            .and_then(|text| <Self as FromFile>::from_json_string(&text))
    }

    #[cfg(feature = "json")]
    fn from_json_string(text: &str) -> Result<Self, Error> {
        serde_json::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "toml")]
    fn from_toml_file(filename: &str) -> Result<Self, Error> {
        <Self as FromFile>::read_file(filename)
            .and_then(|text| <Self as FromFile>::from_toml_string(&text))
    }

    #[cfg(feature = "toml")]
    fn from_toml_string(text: &str) -> Result<Self, Error> {
        toml::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "xml")]
    fn from_xml_file(filename: &str) -> Result<Self, Error> {
        <Self as FromFile>::read_file(filename)
            .and_then(|text| <Self as FromFile>::from_xml_string(&text))
    }

    #[cfg(feature = "xml")]
    fn from_xml_string(text: &str) -> Result<Self, Error> {
        quick_xml::de::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "yaml")]
    fn from_yaml_file(filename: &str) -> Result<Self, Error> {
        <Self as FromFile>::read_file(filename)
            .and_then(|text| <Self as FromFile>::from_yaml_string(&text))
    }

    #[cfg(feature = "yaml")]
    fn from_yaml_string(text: &str) -> Result<Self, Error> {
        serde_yaml::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
    }

    fn read_file(filename: &str) -> Result<String, Error> {
        read_to_string(filename).map_err(Error::FileError)
    }
}

impl<T> FromFile for T where T: for<'de> Deserialize<'de> {}

pub trait ToFile: Serialize + Sized {
    fn write_to_file(&self, filename: &str) -> Result<(), Error> {
        match extension(&PathBuf::from(filename))? {
            #[cfg(feature = "json")]
            "json" => <Self as ToFile>::write_to_json_file(self, filename),
            #[cfg(feature = "toml")]
            "toml" => <Self as ToFile>::write_to_toml_file(self, filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as ToFile>::write_to_xml_file(self, filename),
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => <Self as ToFile>::write_to_yaml_file(self, filename),
            extension => Err(Error::InvalidExtension(Some(extension.to_string()))),
        }
    }

    #[cfg(feature = "json")]
    fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "json")]
    fn write_to_json_file(&self, filename: &str) -> Result<(), Error> {
        <Self as ToFile>::write_file(filename, <Self as ToFile>::to_json(self)?)
    }

    #[cfg(feature = "toml")]
    fn to_toml(&self) -> Result<String, Error> {
        toml::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "toml")]
    fn write_to_toml_file(&self, filename: &str) -> Result<(), Error> {
        <Self as ToFile>::write_file(filename, <Self as ToFile>::to_toml(self)?)
    }

    #[cfg(feature = "xml")]
    fn to_xml(&self) -> Result<String, Error> {
        quick_xml::se::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "xml")]
    fn write_to_xml_file(&self, filename: &str) -> Result<(), Error> {
        <Self as ToFile>::write_file(filename, <Self as ToFile>::to_xml(self)?)
    }

    #[cfg(feature = "yaml")]
    fn to_yaml(&self) -> Result<String, Error> {
        serde_yaml::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
    }

    #[cfg(feature = "yaml")]
    fn write_to_yaml_file(&self, filename: &str) -> Result<(), Error> {
        <Self as ToFile>::write_file(filename, <Self as ToFile>::to_yaml(self)?)
    }

    fn write_file(filename: &str, content: String) -> Result<(), Error> {
        write(filename, content).map_err(Error::FileError)
    }
}

impl<T> ToFile for T where T: Serialize {}

fn extension(path: &Path) -> Result<&str, Error> {
    path.extension()
        .and_then(|extension| extension.to_str())
        .ok_or(Error::InvalidExtension(None))
}
