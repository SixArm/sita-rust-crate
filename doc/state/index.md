# State

State is how Sita holds variables, such a web page front matter markup.

State can use any of these flavors:

* HTML key value lookup, implemented with Rust standard `BTreeMap<String, String>`.

* JSON tree, implemented with Rust Serde JSON `serde_json::Map`.

* TOML table, implemented with Rust Serde TOML `toml::value::Table`.

* YAML tree, implemented with Rust Serde YAML `serde_yaml::Mapping`.

`State` is a trait that provides some simple generic functions.

`StateEnum` is a mutex for the flavors.
