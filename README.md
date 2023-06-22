# serde_rw
A library to extend serde serializers and deserializers with the ability to read / write different file formats from / to files.

## Usage
- To read from files, this crate provides the trait `FromFile`.
- To write to files, this crate provides the trait `ToFile`.

### Default implementations
- `FromFile` is auto-implemented for `serde::Deserialize`.
- `ToFile` is auto-implemented for `serde::Serialize`.

### File formats
Currently the following file formats are supported:

- `JSON` via the `json` feature.
- `TOML` via the `toml` feature.
- `XML` via the `xml` feature.
- `YAML` via the `yaml` feature.

## Credits
This library is inspired by [`from_file`](https://github.com/shakyShane/from_file) by [Shane Osbourne](https://github.com/shakyShane).
