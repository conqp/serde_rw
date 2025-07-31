use std::fs::{read_to_string, write};
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

/// Allow deserialization from TOML.
#[allow(clippy::module_name_repetitions)]
pub trait FromToml: DeserializeOwned {
    /// Deserializes an object from a TOML file
    ///
    /// # Arguments
    /// * `filename` - The path of the TOML file to be read
    ///
    /// # Errors
    /// * `anyhow::Error` - If the file could not be read
    ///
    /// # Examples
    /// ```
    /// use serde_rw::FromToml;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize, Eq, PartialEq)]
    /// struct Person {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// #[cfg(feature = "toml")]
    /// {
    ///     assert_eq!(
    ///         Person::from_toml_file("./tests/person.toml").unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    /// ```
    fn from_toml_file(filename: impl AsRef<Path>) -> crate::Result<Self> {
        <Self as FromToml>::from_toml_string(&read_to_string(filename)?)
    }

    /// Deserializes an object from a TOML string
    ///
    /// # Arguments
    /// * `text` - A TOML file's content
    ///
    /// # Errors
    /// * `anyhow::Error` - If the text could not be deserialized
    ///
    /// # Examples
    /// ```
    /// use serde_rw::FromToml;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, Deserialize, Eq, PartialEq)]
    /// struct Person {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// const TOML: &str = r#"id = 1337
    /// name = "John Doe""#;
    ///
    /// #[cfg(feature = "toml")]
    /// {
    ///     assert_eq!(
    ///         Person::from_toml_string(TOML).unwrap(),
    ///         Person {
    ///             id: 1337,
    ///             name: "John Doe".to_string(),
    ///         }
    ///     );
    /// }
    /// ```
    fn from_toml_string(text: &str) -> crate::Result<Self> {
        Ok(toml::from_str(text)?)
    }
}

/// Allow serialization to TOML.
#[allow(clippy::module_name_repetitions)]
pub trait ToToml: Serialize {
    /// Return object as serialized TOML string
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn to_toml(&self) -> crate::Result<String> {
        Ok(toml::to_string(self)?)
    }

    /// Writes object as serialized TOML string to a file
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn write_to_toml_file(&self, filename: impl AsRef<Path>) -> crate::Result<()> {
        Ok(write(filename, <Self as ToToml>::to_toml(self)?)?)
    }
}
