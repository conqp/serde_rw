#[cfg(feature = "xml")]
pub mod featured {
    use crate::Error;
    use quick_xml::events::Event;
    use quick_xml::{Reader, Writer};
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::fmt::Write;
    use std::fs::{read_to_string, write};

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
        fn from_xml_file(filename: &str) -> Result<Self, Error> {
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
            quick_xml::de::from_str(text).map_err(|error| Error::SerdeError(error.to_string()))
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
                .map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Return object as serialized XML string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_xml(&self) -> Result<String, Error> {
            quick_xml::se::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Return object as a pretty serialized XML string
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn to_xml_pretty(&self, indent_char: char, indent_size: usize) -> Result<String, Error> {
            let xml = self.to_xml()?;
            let mut reader = Reader::from_str(&xml);
            reader.trim_text(true);
            let mut writer = Writer::new_with_indent(Vec::new(), indent_char as u8, indent_size);

            loop {
                let ev = reader.read_event();

                match ev {
                    Ok(Event::Eof) => break,
                    Ok(event) => {
                        if let Err(error) = writer.write_event(event) {
                            return Err(Error::SerdeError(error.to_string()));
                        }
                    }
                    Err(e) => return Err(Error::SerdeError(e.to_string())),
                }
            }

            String::from_utf8(writer.into_inner())
                .map_err(|error| Error::SerdeError(error.to_string()))
        }

        /// Writes object as serialized XML string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_xml_file(&self, filename: &str) -> Result<(), Error> {
            write(filename, <Self as ToXml>::to_xml(self)?).map_err(Error::FileError)
        }

        /// Writes object as a pretty serialized XML string to a file
        /// # Errors
        /// Returns an `serde_rw::Error` in case the serialization fails.
        fn write_to_xml_file_pretty(
            &self,
            filename: &str,
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
