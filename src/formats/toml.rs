#[cfg(feature = "toml")]
pub mod featured {
    use crate::Error;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::fs::{read_to_string, write};

    pub trait FromToml: DeserializeOwned {
        /// Deserializes an object from a TOML file
        /// # Arguments
        /// * `filename` - The path of the TOML file to be read
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the file could not be read
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
        fn from_toml_file(filename: &str) -> Result<Self, Error> {
            read_to_string(filename)
                .map_err(Error::FileError)
                .and_then(|text| <Self as FromToml>::from_toml_string(&text))
        }

        /// Deserializes an object from a TOML string
        /// # Arguments
        /// * `text` - A TOML file's content
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the text could not be deserialized
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
        fn from_toml_string(text: &str) -> Result<Self, Error> {
            toml::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
        }
    }

    pub trait ToToml: Serialize {
        /// Return object as serialized TOML string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_toml(&self) -> Result<String, Error> {
            toml::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Writes object as serialized TOML string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_toml_file(&self, filename: &str) -> Result<(), Error> {
            write(filename, <Self as ToToml>::to_toml(self)?).map_err(Error::FileError)
        }
    }
}
