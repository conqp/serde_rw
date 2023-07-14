#[cfg(feature = "json")]
pub mod featured {
    use crate::Error;
    use serde::{Deserialize, Serialize};
    use std::fs::{read_to_string, write};
    use std::io::{BufWriter, Write};
    use std::path::Path;

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
        fn from_json_file(filename: impl AsRef<Path>) -> Result<Self, Error> {
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
        /// Write object as JSON to a `std::io::Write`r
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_json<W>(&self, writer: W) -> Result<(), Error>
        where
            W: Write,
        {
            serde_json::to_writer(writer, self)
                .map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Write object as pretty JSON to a `std::io::Write`
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_json_pretty<W>(&self, writer: W) -> Result<(), Error>
        where
            W: Write,
        {
            serde_json::to_writer_pretty(writer, self)
                .map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Return object as serialized JSON string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_json(&self) -> Result<String, Error> {
            serde_json::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Return object as prettified JSON string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_json_pretty(&self) -> Result<String, Error> {
            let mut writer = BufWriter::new(Vec::new());
            <Self as ToJson>::write_json_pretty(self, &mut writer)
                .map_err(|error| Error::SerdeError(error.to_string()))?;
            String::from_utf8(
                writer
                    .into_inner()
                    .map_err(|error| Error::SerdeError(error.to_string()))?,
            )
            .map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Write object as serialized JSON string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_json_file(&self, filename: impl AsRef<Path>) -> Result<(), Error> {
            write(filename, <Self as ToJson>::to_json(self)?).map_err(Error::FileError)
        }

        /// Write object as serialized JSON string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_json_file_pretty(&self, filename: impl AsRef<Path>) -> Result<(), Error> {
            write(filename, <Self as ToJson>::to_json_pretty(self)?).map_err(Error::FileError)
        }
    }
}
