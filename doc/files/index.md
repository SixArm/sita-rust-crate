# Files

* `src/`
    * `main.rs` - Main function that loads everything; calls `run.rs`.
    * `types.rs` - Type aliases and related macros.
    * `app/` - Application files typical of any of our apps
        * `args.rs` - Arguments as a struct; can be created by `clap.rs`.
        * `clap.rs` - Command line argument parsing, which creates an `args` struct.
        * `config.rs` - Configuration struct, such as set via `confy`
        * `confy.rs` - Configuration file parsing tests, which load configuration variables.
        * `run.rs` - Run function that does the core business logic; called by `main.rs`.
    * `f/` - Functions, including utilties, helpers, converters, etc.
    * `markdown/` - Mardown-related
    * `matter/` - Markdown front matter and back matter files.
        * `matter_parser_enum.rs` - Matter parser enum (among BTMS, JSON, TOML, YAML).
        * `matter_parser_mutex.rs` - Matter parser mutex (among BTMS, JSON, TOML, YAML).
        * `matter_parser_trait.rs` - Matter parser trait (among `matter_parser_with_*`).
        * `matter_parser_with_btms.rs` - Matter parser implementation with BTMS (BTreeMap Struct).
        * `matter_parser_with_json.rs` - Matter parser implementation with JSON (JavaScript Object Notation).
        * `matter_parser_with_toml.rs` - Matter parser implementation with TOML (Tom's Obvious Minimal Language).
        * `matter_parser_with_yaml.rs` - Matter parser implementation with YAML (Yet Another Markup Language).
    * `state/` - State that holds variables, such as front matter.
        * `state_enum.rs` - State enum (among BTMS, JSON, TOML, YAML).
        * `state_trait.rs` - State trait (among `state_with_*.rs`).
        * `state_with_btms.rs` - State implementation with BTMS (BTreeMap Struct).
        * `state_with_json.rs` - State implementation with JSON (JavaScript Object Notation).
        * `state_with_toml.rs` - State implementation with TOML (Tom's Obvious Minimal Language).
        * `state_with_yaml.rs` - State implementation with YAML (Yet Another Markup Language).
    * `templating/` - Templating-related
        * `templater.rs` - Templater trait, implemented by `templater_with_*.rs`.
        * `templater_with_handlebars.rs` - Templater implementation with Handlebars.
        * `templater_with_tera.rs` - Templater implementation with Tera.
* `tests/` - Tests of the system
    * `command/` - Command tests that run the app with various args
    * `f/` - Function tests, such as function example data and example files.
    * `markdown/` - Markdown tests, such as parsing, including front matter kinds
    * `src/` - Source test extras, such as example data and example files.
    * `tutorial/` - Tutorial tests that match the README documentation
