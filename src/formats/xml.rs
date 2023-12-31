use quick_xml::se::Serializer;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Write;
use std::fs::{read_to_string, write};
use std::path::Path;

#[allow(clippy::module_name_repetitions)]
pub trait FromXml: DeserializeOwned {
    /// Deserializes an object from an XML file
    ///
    /// # Arguments
    /// * `filename` - The path of the XML file to be read
    ///
    /// # Errors
    /// * `anyhow::Error` - If the file could not be read
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
    fn from_xml_file(filename: impl AsRef<Path>) -> anyhow::Result<Self> {
        <Self as FromXml>::from_xml_string(&read_to_string(filename)?)
    }

    /// Deserializes an object from an XML string
    ///
    /// # Arguments
    /// * `text` - An XML file's content
    ///
    /// # Errors
    /// * `anyhow::Error` - If the text could not be deserialized
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
    fn from_xml_string(text: &str) -> anyhow::Result<Self> {
        Ok(quick_xml::de::from_str(text)?)
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait ToXml: Serialize {
    /// Write object as XML to a `std::fmt::Write`r
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn write_xml<W>(&self, writer: W) -> anyhow::Result<()>
    where
        W: Write,
    {
        Ok(quick_xml::se::to_writer(writer, self)?)
    }

    /// Return object as serialized XML string
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn to_xml(&self) -> anyhow::Result<String> {
        Ok(quick_xml::se::to_string(self)?)
    }

    /// Return object as a pretty serialized XML string
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn to_xml_pretty(&self, indent_char: char, indent_size: usize) -> anyhow::Result<String> {
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.indent(indent_char, indent_size);
        self.serialize(serializer)?;
        Ok(buffer)
    }

    /// Writes object as serialized XML string to a file
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn write_to_xml_file(&self, filename: impl AsRef<Path>) -> anyhow::Result<()> {
        Ok(write(filename, <Self as ToXml>::to_xml(self)?)?)
    }

    /// Writes object as a pretty serialized XML string to a file
    ///
    /// # Errors
    /// Returns an `anyhow::Error` in case the serialization fails.
    fn write_to_xml_file_pretty(
        &self,
        filename: impl AsRef<Path>,
        indent_char: char,
        indent_size: usize,
    ) -> anyhow::Result<()> {
        Ok(write(
            filename,
            <Self as ToXml>::to_xml_pretty(self, indent_char, indent_size)?,
        )?)
    }
}
