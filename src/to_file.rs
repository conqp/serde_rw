use crate::functions::extension;
use crate::Error;
use serde::Serialize;
use std::path::PathBuf;

/// Makes an object capable of writing itself to a file of a specified format
pub trait ToFile: Serialize + Sized {
    /// Serializes an object into a file dependent on its file extension
    /// # Arguments
    /// * `filename` - The path of the file to be written to
    ///
    /// # Errors
    /// * `serde_rw::Error` - if any serialization or I/O error occur or the file format is not supported
    fn write_to_file(&self, filename: &str) -> Result<(), Error> {
        match extension(&PathBuf::from(filename))? {
            #[cfg(feature = "json")]
            "json" => <Self as crate::ToJson>::write_to_json_file(self, filename),
            #[cfg(feature = "toml")]
            "toml" => <Self as crate::ToToml>::write_to_toml_file(self, filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as crate::ToXml>::write_to_xml_file(self, filename),
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => <Self as crate::ToYaml>::write_to_yaml_file(self, filename),
            extension => Err(Error::InvalidExtension(Some(extension.to_string()))),
        }
    }

    /// Serializes an object into a prettified file dependent on its file extension
    /// # Arguments
    /// * `filename` - The path of the file to be written to
    ///
    /// # Errors
    /// * `serde_rw::Error` - if any serialization or I/O error occur or the file format is not supported
    fn write_to_file_pretty(&self, filename: &str) -> Result<(), Error> {
        match extension(&PathBuf::from(filename))? {
            #[cfg(feature = "json")]
            "json" => <Self as crate::ToJson>::write_to_json_file_pretty(self, filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as crate::ToXml>::write_to_xml_file(self, filename),
            extension => Err(Error::InvalidExtension(Some(extension.to_string()))),
        }
    }
}

impl<T> ToFile for T where T: Serialize {}
#[cfg(feature = "json")]
impl<T> crate::ToJson for T where T: ToFile {}
#[cfg(feature = "toml")]
impl<T> crate::ToToml for T where T: ToFile {}
#[cfg(feature = "xml")]
impl<T> crate::ToXml for T where T: ToFile {}
#[cfg(feature = "yaml")]
impl<T> crate::ToYaml for T where T: ToFile {}
