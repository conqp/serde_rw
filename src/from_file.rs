use std::ffi::OsStr;
use std::path::Path;

use serde::Deserialize;

use crate::Error;

/// Makes an object capable of reading itself from a file of a specified format
pub trait FromFile
where
    for<'de> Self: Deserialize<'de>,
{
    /// Deserializes an object from a file dependent on its file extension
    ///
    /// # Arguments
    /// * `filename` - The path of the file to be read
    ///
    /// # Errors
    /// * `anyhow::Error` - If the file could not be read
    ///
    /// # Examples
    /// ```
    /// use serde_rw::FromFile;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize, Eq, PartialEq)]
    /// struct Person {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// // Read JSON files with the `json` feature:
    /// #[cfg(feature = "json")]
    /// {
    ///     use serde_rw::FromFile;
    ///     assert_eq!(
    ///         Person::from_file("./tests/person.json").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    ///
    /// // Read TOML files with the `toml` feature:
    /// #[cfg(feature = "toml")]
    /// {
    ///     use serde_rw::FromFile;
    ///     assert_eq!(
    ///         Person::from_file("./tests/person.toml").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    ///
    /// // Read XML files with the `xml` feature:
    /// #[cfg(feature = "xml")]
    /// {
    ///     use serde_rw::FromFile;
    ///     assert_eq!(
    ///         Person::from_file("./tests/person.xml").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    ///
    /// // Read YAML files with the `yaml` feature:
    /// #[cfg(feature = "yaml")]
    /// {
    ///     use serde_rw::FromFile;
    ///     assert_eq!(
    ///         Person::from_file("./tests/person.yml").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    /// ```
    fn from_file(filename: impl AsRef<Path>) -> crate::Result<Self> {
        let extension = filename
            .as_ref()
            .extension()
            .map(OsStr::to_ascii_lowercase)
            .ok_or(Error::NoFileExtensionsSpecified)?;

        match extension.as_encoded_bytes() {
            #[cfg(feature = "json")]
            b"json" => <Self as crate::FromJson>::from_json_file(filename),
            #[cfg(feature = "toml")]
            b"toml" => <Self as crate::FromToml>::from_toml_file(filename),
            #[cfg(feature = "xml")]
            b"xml" => <Self as crate::FromXml>::from_xml_file(filename),
            #[cfg(feature = "yaml")]
            b"yml" | b"yaml" => <Self as crate::FromYaml>::from_yaml_file(filename),
            _ => Err(Error::UnsupportedFileExtension(extension)),
        }
    }
}

impl<T> FromFile for T where T: for<'de> Deserialize<'de> {}

#[cfg(feature = "json")]
impl<T> crate::FromJson for T where T: FromFile {}
#[cfg(feature = "toml")]
impl<T> crate::FromToml for T where T: FromFile {}
#[cfg(feature = "xml")]
impl<T> crate::FromXml for T where T: FromFile {}
#[cfg(feature = "yaml")]
impl<T> crate::FromYaml for T where T: FromFile {}
