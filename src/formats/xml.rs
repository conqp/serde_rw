use std::fmt::Write;
use std::fs::{read_to_string, write};
use std::path::Path;

use quick_xml::se::Serializer;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Allow deserialization from XML.
#[allow(clippy::module_name_repetitions)]
pub trait FromXml: DeserializeOwned {
    /// Deserializes an object from an XML file.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the deserialization fails.
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
    fn from_xml_file(filename: impl AsRef<Path>) -> crate::Result<Self> {
        <Self as FromXml>::from_xml_string(&read_to_string(filename)?)
    }

    /// Deserializes an object from an XML string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the deserialization fails.
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
    fn from_xml_string(text: &str) -> crate::Result<Self> {
        Ok(quick_xml::de::from_str(text)?)
    }
}

/// Allow serialization to XML.
#[allow(clippy::module_name_repetitions)]
pub trait ToXml: Serialize {
    /// Write object as XML to a [writer](Write).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn write_xml<W>(&self, writer: W) -> crate::Result<()>
    where
        W: Write,
    {
        Ok(quick_xml::se::to_writer(writer, self).map(drop)?)
    }

    /// Return object as serialized XML string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn to_xml(&self) -> crate::Result<String> {
        Ok(quick_xml::se::to_string(self)?)
    }

    /// Return object as a pretty serialized XML string.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn to_xml_pretty(&self, indent_char: char, indent_size: usize) -> crate::Result<String> {
        let mut buffer = String::new();
        let mut serializer = Serializer::new(&mut buffer);
        serializer.indent(indent_char, indent_size);
        self.serialize(serializer)?;
        Ok(buffer)
    }

    /// Writes object as serialized XML string to a file.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn write_to_xml_file(&self, filename: impl AsRef<Path>) -> crate::Result<()> {
        Ok(write(filename, <Self as ToXml>::to_xml(self)?)?)
    }

    /// Writes object as a pretty serialized XML string to a file.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`](crate::Error) if the serialization fails.
    fn write_to_xml_file_pretty(
        &self,
        filename: impl AsRef<Path>,
        indent_char: char,
        indent_size: usize,
    ) -> crate::Result<()> {
        Ok(write(
            filename,
            <Self as ToXml>::to_xml_pretty(self, indent_char, indent_size)?,
        )?)
    }
}
