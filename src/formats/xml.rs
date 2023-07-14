#[cfg(feature = "xml")]
pub mod featured {
    use crate::Error;
    use quick_xml::se::Serializer;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::fmt::Write;
    use std::fs::{read_to_string, write};
    use std::path::Path;

    pub trait FromXml: DeserializeOwned {
        /// Deserializes an object from an XML file
        /// # Arguments
        /// * `filename` - The path of the XML file to be read
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the file could not be read
        ///
        /// # Examples
        /// ```
        /// use serde_rw::FromXml;
        /// use serde::Deserialize;
        ///
        /// #[derive(Debug, Deserialize, Eq, PartialEq)]
        /// struct Person {
        ///     id: u32,
        ///     name: String,
        /// }
        ///
        /// #[cfg(feature = "xml")]
        /// {
        ///     assert_eq!(
        ///         Person::from_xml_file("./tests/person.xml").unwrap(),
        ///         Person {
        ///             id: 1337,
        ///             name: "John Doe".to_string(),
        ///         }
        ///     );
        /// }
        /// ```
        fn from_xml_file(filename: impl AsRef<Path>) -> Result<Self, Error> {
            read_to_string(filename)
                .map_err(Error::FileError)
                .and_then(|text| <Self as FromXml>::from_xml_string(&text))
        }

        /// Deserializes an object from an XML string
        /// # Arguments
        /// * `text` - An XML file's content
        ///
        /// # Errors
        /// * `serde_rw::Error` - If the text could not be deserialized
        ///
        /// # Examples
        /// ```
        /// use serde_rw::FromXml;
        /// use serde::Deserialize;
        ///
        /// #[derive(Debug, Deserialize, Eq, PartialEq)]
        /// struct Person {
        ///     id: u32,
        ///     name: String,
        /// }
        ///
        /// const XML: &str = "<person><id>1337</id><name>John Doe</name></person>";
        ///
        /// #[cfg(feature = "xml")]
        /// {
        ///     assert_eq!(
        ///         Person::from_xml_string(XML).unwrap(),
        ///         Person {
        ///             id: 1337,
        ///             name: "John Doe".to_string(),
        ///         }
        ///     );
        /// }
        /// ```
        fn from_xml_string(text: &str) -> Result<Self, Error> {
            quick_xml::de::from_str(text).map_err(|error| Error::DeserializationError(error.into()))
        }
    }

    pub trait ToXml: Serialize {
        /// Write object as XML to a `std::fmt::Write`r
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_xml<W>(&self, writer: W) -> Result<(), Error>
        where
            W: Write,
        {
            quick_xml::se::to_writer(writer, self)
                .map_err(|error| Error::SerializationError(error.into()))
        }

        /// Return object as serialized XML string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_xml(&self) -> Result<String, Error> {
            quick_xml::se::to_string(self).map_err(|error| Error::SerializationError(error.into()))
        }

        /// Return object as a pretty serialized XML string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_xml_pretty(&self, indent_char: char, indent_size: usize) -> Result<String, Error> {
            let mut buffer = String::new();
            let mut serializer = Serializer::new(&mut buffer);
            serializer.indent(indent_char, indent_size);
            self.serialize(serializer)
                .map_err(|error| Error::SerializationError(error.into()))?;
            Ok(buffer)
        }

        /// Writes object as serialized XML string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_xml_file(&self, filename: impl AsRef<Path>) -> Result<(), Error> {
            write(filename, <Self as ToXml>::to_xml(self)?).map_err(Error::FileError)
        }

        /// Writes object as a pretty serialized XML string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_xml_file_pretty(
            &self,
            filename: impl AsRef<Path>,
            indent_char: char,
            indent_size: usize,
        ) -> Result<(), Error> {
            write(
                filename,
                <Self as ToXml>::to_xml_pretty(self, indent_char, indent_size)?,
            )
            .map_err(Error::FileError)
        }
    }
}
