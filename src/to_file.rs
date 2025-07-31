use std::path::Path;

use anyhow::anyhow;
use serde::Serialize;

use crate::functions::extension;

/// Makes an object capable of writing itself to a file of a specified format
pub trait ToFile: Serialize + Sized {
    /// Serializes an object into a file dependent on its file extension
    ///
    /// # Arguments
    /// * `filename` - The path of the file to be written to
    ///
    /// # Errors
    /// * `anyhow::Error` - if any serialization or I/O errors occur or the file format is not supported
    fn write_to_file(&self, filename: impl AsRef<Path>) -> anyhow::Result<()> {
        match extension(filename.as_ref())? {
            #[cfg(feature = "json")]
            "json" => <Self as crate::ToJson>::write_to_json_file(self, filename),
            #[cfg(feature = "toml")]
            "toml" => <Self as crate::ToToml>::write_to_toml_file(self, filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as crate::ToXml>::write_to_xml_file(self, filename),
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => <Self as crate::ToYaml>::write_to_yaml_file(self, filename),
            extension => Err(anyhow!("Unsupported extension: '{extension}'")),
        }
    }

    /// Serializes an object into a prettified file dependent on its file extension
    ///
    /// # Arguments
    /// * `filename` - The path of the file to be written to
    ///
    /// # Errors
    /// * `anyhow::Error` - if any serialization or I/O errors occur or the file format is not supported
    fn write_to_file_pretty(&self, filename: impl AsRef<Path>) -> anyhow::Result<()> {
        match extension(filename.as_ref())? {
            #[cfg(feature = "json")]
            "json" => <Self as crate::ToJson>::write_to_json_file_pretty(self, filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as crate::ToXml>::write_to_xml_file_pretty(self, filename, ' ', 4),
            _ => self.write_to_file(filename),
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
