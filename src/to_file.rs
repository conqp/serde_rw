use std::ffi::OsStr;
use std::path::Path;

use serde::Serialize;

use crate::Error;

#[cfg(feature = "xml")]
const XML_INDENT_CHAR: char = ' ';
#[cfg(feature = "xml")]
const XML_INDENT_LEN: usize = 4;

/// Makes an object capable of writing itself to a file of a specified format
pub trait ToFile: Serialize + Sized {
    /// Serializes an object into a file dependent on its file extension
    ///
    /// # Arguments
    /// * `filename` - The path of the file to be written to
    ///
    /// # Errors
    /// * `anyhow::Error` - if any serialization or I/O errors occur or the file format is not supported
    fn write_to_file(&self, filename: impl AsRef<Path>) -> crate::Result<()> {
        let extension = filename
            .as_ref()
            .extension()
            .map(OsStr::to_ascii_lowercase)
            .ok_or(Error::NoFileExtensionsSpecified)?;

        match extension.as_encoded_bytes() {
            #[cfg(feature = "json")]
            b"json" => <Self as crate::ToJson>::write_to_json_file(self, filename),
            #[cfg(feature = "toml")]
            b"toml" => <Self as crate::ToToml>::write_to_toml_file(self, filename),
            #[cfg(feature = "xml")]
            b"xml" => <Self as crate::ToXml>::write_to_xml_file(self, filename),
            #[cfg(feature = "yaml")]
            b"yml" | b"yaml" => <Self as crate::ToYaml>::write_to_yaml_file(self, filename),
            _ => Err(Error::UnsupportedFileExtension(extension)),
        }
    }

    /// Serializes an object into a prettified file dependent on its file extension
    ///
    /// # Arguments
    /// * `filename` - The path of the file to be written to
    ///
    /// # Errors
    /// * `anyhow::Error` - if any serialization or I/O errors occur or the file format is not supported
    fn write_to_file_pretty(&self, filename: impl AsRef<Path>) -> crate::Result<()> {
        let extension = filename
            .as_ref()
            .extension()
            .map(OsStr::to_ascii_lowercase)
            .ok_or(Error::NoFileExtensionsSpecified)?;

        match extension.as_encoded_bytes() {
            #[cfg(feature = "json")]
            b"json" => <Self as crate::ToJson>::write_to_json_file_pretty(self, filename),
            #[cfg(feature = "xml")]
            b"xml" => <Self as crate::ToXml>::write_to_xml_file_pretty(
                self,
                filename,
                XML_INDENT_CHAR,
                XML_INDENT_LEN,
            ),
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
