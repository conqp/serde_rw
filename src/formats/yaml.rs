#[cfg(feature = "yaml")]
pub mod featured {
    use serde::{Deserialize, Serialize};
    use std::fs::{read_to_string, write};
    use std::path::Path;

    pub trait FromYaml: for<'de> Deserialize<'de> {
        /// Deserializes an object from a YAML file
        ///
        /// # Arguments
        /// * `filename` - The path of the YAML file to be read
        ///
        /// # Errors
        /// * `anyhow::Error` - If the file could not be read
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
        fn from_yaml_file(filename: impl AsRef<Path>) -> anyhow::Result<Self> {
            <Self as FromYaml>::from_yaml_string(&read_to_string(filename)?)
        }

        /// Deserializes an object from a YAML string
        ///
        /// # Arguments
        /// * `text` - A YAML file's content
        ///
        /// # Errors
        /// * `anyhow::Error` - If the text could not be deserialized
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
        fn from_yaml_string(text: &str) -> anyhow::Result<Self> {
            Ok(serde_yaml::from_str(text)?)
        }
    }

    pub trait ToYaml: Serialize {
        /// Return object as serialized YAML string
        ///
        /// # Errors
        /// Returns an `anyhow::Error` in case the serialization fails.
        fn to_yaml(&self) -> anyhow::Result<String> {
            Ok(serde_yaml::to_string(self)?)
        }

        /// Writes object as serialized YAML string to a file
        ///
        /// # Errors
        /// Returns an `anyhow::Error` in case the serialization fails.
        fn write_to_yaml_file(&self, filename: impl AsRef<Path>) -> anyhow::Result<()> {
            Ok(write(filename, <Self as ToYaml>::to_yaml(self)?)?)
        }
    }
}
