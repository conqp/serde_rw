#[cfg(feature = "yaml")]
pub mod featured {
    use crate::Error;
    use serde::{Deserialize, Serialize};
    use std::fs::{read_to_string, write};

    pub trait FromYaml<'de>: Deserialize<'de> {
        /// Deserializes an object from a YAML file
        /// # Arguments
        /// * `filename` - The path of the YAML file to be read
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
        fn from_yaml_file(filename: &str) -> Result<Self, Error> {
            read_to_string(filename)
                .map_err(Error::FileError)
                .and_then(|text| <Self as FromYaml>::from_yaml_string(&text))
        }

        /// Deserializes an object from a YAML string
        /// # Arguments
        /// * `text` - A YAML file's content
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the text could not be deserialized
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
        fn from_yaml_string(text: &str) -> Result<Self, Error> {
            serde_yaml::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
        }
    }

    pub trait ToYaml: Serialize {
        fn to_yaml(&self) -> Result<String, Error> {
            serde_yaml::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
        }

        fn write_to_yaml_file(&self, filename: &str) -> Result<(), Error> {
            write(filename, <Self as ToYaml>::to_yaml(self)?).map_err(Error::FileError)
        }
    }
}
