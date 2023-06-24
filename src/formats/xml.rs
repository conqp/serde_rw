#[cfg(feature = "xml")]
pub mod featured {
    use crate::Error;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
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
        /// use serde_rw::FromFile;
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
        /// use serde_rw::FromFile;
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
        fn to_xml(&self) -> Result<String, Error> {
            quick_xml::se::to_string(self).map_err(|error| Error::SerdeError(error.to_string()))
        }

        fn write_to_xml_file(&self, filename: &str) -> Result<(), Error> {
            write(filename, <Self as ToXml>::to_xml(self)?).map_err(Error::FileError)
        }
    }
}
