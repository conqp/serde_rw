use std::fs::{read_to_string, write};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Allow deserialization from YAML.
#[allow(clippy::module_name_repetitions)]
pub trait FromYaml: for<'de> Deserialize<'de> {
    /// Deserializes an object from a YAML file.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the deserialization fails.
    ///
    /// # Examples
    /// ```
    /// use serde_rw::FromYaml;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize, Eq, PartialEq)]
    /// struct Person {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// #[cfg(feature = "yaml")]
    /// {
    ///     assert_eq!(
    ///         Person::from_yaml_file("./tests/person.yml").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    /// ```
    fn from_yaml_file(filename: impl AsRef<Path>) -> crate::Result<Self> {
        <Self as FromYaml>::from_yaml_string(&read_to_string(filename)?)
    }

    /// Deserializes an object from a YAML string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the deserialization fails.
    ///
    /// # Examples
    /// ```
    /// use serde_rw::FromYaml;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize, Eq, PartialEq)]
    /// struct Person {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// const YAML: &str = r#"id: 1337
    /// name: "John Doe""#;
    ///
    /// #[cfg(feature = "yaml")]
    /// {
    ///     assert_eq!(
    ///         Person::from_yaml_string(YAML).unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    /// ```
    fn from_yaml_string(text: &str) -> crate::Result<Self> {
        Ok(serde_yaml::from_str(text)?)
    }
}

/// Allow serialization to YAML.
#[allow(clippy::module_name_repetitions)]
pub trait ToYaml: Serialize {
    /// Return object as serialized YAML string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn to_yaml(&self) -> crate::Result<String> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Writes object as serialized YAML string to a file.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn write_to_yaml_file(&self, filename: impl AsRef<Path>) -> crate::Result<()> {
        Ok(write(filename, <Self as ToYaml>::to_yaml(self)?)?)
    }
}
