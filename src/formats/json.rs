#[cfg(feature = "json")]
pub mod featured {
    use crate::Error;
    use serde::{Deserialize, Serialize};
    use std::fs::{read_to_string, write};
    use std::io::Write;

    pub trait FromJson: for<'de> Deserialize<'de> {
        /// Deserializes an object from a JSON file
        /// # Arguments
        /// * `filename` - The path of the JSON file to be read
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the file could not be read
        ///
        /// # Examples
        /// ```
        /// use serde_rw::FromJson;
        /// use serde::Deserialize;
        ///
        /// #[derive(Debug, Deserialize, Eq, PartialEq)]
        /// struct Person {
        ///     id: u32,
        ///     name: String,
        /// }
        ///
        /// #[cfg(feature = "json")]
        /// {
        ///     assert_eq!(
        ///         Person::from_json_file("./tests/person.json").unwrap(),
        ///         Person {
        ///             id: 1337,
        ///             name: "John Doe".to_string(),
        ///         }
        ///     );
        /// }
        /// ```
        fn from_json_file(filename: &str) -> Result<Self, Error> {
            read_to_string(filename)
                .map_err(Error::FileError)
                .and_then(|text| <Self as FromJson>::from_json_string(&text))
        }

        /// Deserializes an object from a JSON string
        /// # Arguments
        /// * `text` - A JSON file's content
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the text could not be deserialized
        ///
        /// # Examples
        /// ```
        /// use serde_rw::FromJson;
        /// use serde::Deserialize;
        ///
        /// #[derive(Debug, Deserialize, Eq, PartialEq)]
        /// struct Person {
        ///     id: u32,
        ///     name: String,
        /// }
        ///
        /// const JSON: &str = r#"{"id": 1337, "name": "John Doe"}"#;
        ///
        /// #[cfg(feature = "json")]
        /// {
        ///     assert_eq!(
        ///         Person::from_json_string(JSON).unwrap(),
        ///         Person {
        ///             id: 1337,
        ///             name: "John Doe".to_string(),
        ///         }
        ///     );
        /// }
        /// ```
        fn from_json_string(text: &str) -> Result<Self, Error> {
            serde_json::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
        }
    }

    pub trait ToJson: Serialize {
        fn write_json<W>(&self, writer: W) -> Result<(), Error>
        where
            W: Write,
        {
            serde_json::to_writer(writer, self)
                .map_err(|error| Error::SerdeError(error.to_string()))
        }

        fn write_json_pretty<W>(&self, writer: W) -> Result<(), Error>
        where
            W: Write,
        {
            serde_json::to_writer_pretty(writer, self)
                .map_err(|error| Error::SerdeError(error.to_string()))
        }

        fn to_json(&self) -> Result<String, Error> {
            serde_json::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
        }

        fn write_to_json_file(&self, filename: &str) -> Result<(), Error> {
            write(filename, <Self as ToJson>::to_json(self)?).map_err(Error::FileError)
        }
    }
}
