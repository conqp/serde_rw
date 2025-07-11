//! A library to extend serde serializers and deserializers with the ability
//! to read / write different file formats from / to files.

pub use from_file::FromFile;
pub use to_file::ToFile;

#[cfg(feature = "json")]
pub use formats::json::{FromJson, ToJson};

#[cfg(feature = "toml")]
pub use formats::toml::{FromToml, ToToml};

#[cfg(feature = "xml")]
pub use formats::xml::{FromXml, ToXml};

#[cfg(feature = "yaml")]
pub use formats::yaml::{FromYaml, ToYaml};

mod formats;
mod from_file;
mod functions;
mod to_file;
