use crate::functions::extension;
use crate::Error;
use serde::Deserialize;
use std::path::PathBuf;

/// Makes an object capable of reading itself from a file of a specified format
pub trait FromFile
where
    for<'de> Self: Deserialize<'de>,
{
    /// Deserializes an object from a file dependent on its file extension
    /// # Arguments
    /// * `filename` - The path of the file to be read
    ///
    /// # Errors
    /// * `serde_rw::Error` - If the file could not be read
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
    ///     assert_eq!(
    ///         Person::from_file("./tests/person.yml").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    /// ```
    fn from_file(filename: &str) -> Result<Self, Error> {
        match extension(&PathBuf::from(filename))? {
            #[cfg(feature = "json")]
            "json" => <Self as crate::FromJson>::from_json_file(filename),
            #[cfg(feature = "toml")]
            "toml" => <Self as crate::FromToml>::from_toml_file(filename),
            #[cfg(feature = "xml")]
            "xml" => <Self as crate::FromXml>::from_xml_file(filename),
            #[cfg(feature = "yaml")]
            "yml" | "yaml" => <Self as crate::FromYaml>::from_yaml_file(filename),
            extension => Err(Error::InvalidExtension(Some(extension.to_string()))),
        }
    }
}

impl<T> FromFile for T where T: for<'de> Deserialize<'de> {}

#[cfg(feature = "json")]
impl<'de, T> crate::FromJson<'de> for T where T: FromFile {}
#[cfg(feature = "toml")]
impl<T> crate::FromToml for T where T: FromFile {}
#[cfg(feature = "xml")]
impl<T> crate::FromXml for T where T: FromFile {}
#[cfg(feature = "yaml")]
impl<'de, T> crate::FromYaml<'de> for T where T: FromFile {}
