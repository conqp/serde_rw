//! A library to extend serde serializers and deserializers with the ability
//! to read / write different file formats from / to files.

pub use error::Error;
#[cfg(feature = "json")]
pub use formats::json::{FromJson, ToJson};
#[cfg(feature = "toml")]
pub use formats::toml::{FromToml, ToToml};
#[cfg(feature = "xml")]
pub use formats::xml::{FromXml, ToXml};
#[cfg(feature = "yaml")]
pub use formats::yaml::{FromYaml, ToYaml};
pub use from_file::FromFile;
pub use to_file::ToFile;

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;

mod error;
mod formats;
mod from_file;
mod to_file;
