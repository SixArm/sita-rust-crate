# State

State is how Sita holds variables, such a web page front matter markup.

State can use any of these flavors:

* HTML key value lookup, implemented as a Rust Map.

* JSON tree, implemented as a Rust Serde JSON Value.

* TOML table, implemented as a Rust Serde TOML Value.

* YAML tree, implemented as a Rust Serde YAML Value.
