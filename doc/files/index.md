# Files

File in order of importance for learning:

* `src/`
    * `main.rs` - Main function that loads everything; calls `run.rs`.
    * `types.rs` - Type aliases and related macros.
    * `app/` - Application files typical of any of our apps
      * `args.rs` - Arguments as a struct; can be created by `clap.rs`.
      * `clap.rs` - Command line argument parsing, which creates an `args` struct.
      * `config.rs` - Configuration struct, such as set via `confy`
      * `confy.rs` - Configuration file parsing tests, which load configuration variables.
      * `run.rs` - Run function that does the core business logic; called by `main.rs`.
    * `f/` - Functions, including utilities, helpers, converters, etc.
    * `markdown/` - Markdown-related
      * `markdown_parser.rs` - Markdown parser using pulldown cmark with the options we prefer.
    * `matter/` - Markdown front matter and back matter files.
      * `matter_parser_enum.rs` - Matter parser enum (among BTMS, JSON, TOML, YAML).
      * `matter_parser_mutex.rs` - Matter parser mutex (among BTMS, JSON, TOML, YAML).
      * `matter_parser_trait.rs` - Matter parser trait (implemented by `matter_parser_with_*.rs`).
      * `matter_parser_with_html.rs` - Matter parser implementation with BTMS (BTreeMap struct).
      * `matter_parser_with_json.rs` - Matter parser implementation with JSON (JavaScript Object Notation).
      * `matter_parser_with_markdown_comments.rs` - Matter parser implementation with MDCC (Markdown comment code).
      * `matter_parser_with_toml.rs` - Matter parser implementation with TOML (Tom's Obvious Minimal Language).
      * `matter_parser_with_yaml.rs` - Matter parser implementation with YAML (Yet Another Markup Language).
    * `state/` - State that holds variables, such as front matter.
      * `state_enum.rs` - State enum (among BTMS, JSON, TOML, YAML).
      * `state_trait.rs` - State trait (among `state_with_*.rs`).
      * `state_with_map.rs` - State implementation with BTMS (BTreeMap struct).
      * `state_with_json.rs` - State implementation with JSON (JavaScript Object Notation).
      * `state_with_toml.rs` - State implementation with TOML (Tom's Obvious Minimal Language).
      * `state_with_yaml.rs` - State implementation with YAML (Yet Another Markup Language).
    * `templater/` - Template processor implementations
      * `templater_enum.rs` - Templater enum, implemented by `templater_with_*.rs`.
      * `templater_trait.rs` - Templater trait (implemented by `templater_with_*.rs`)
      * `templater_with_handlebars.rs` - Templater implementation with Handlebars.
      * `templater_with_tera.rs` - Templater implementation with Tera (TODO).
      * `templater_with_liquid.rs` - Templater implementation with Liquid (TODO).
    * `templating/` - Templating utilities.
      * `serde.rs` - Serde tests of serialization/deserialization.
      * `tags.rs` - Simple functions for building HTML page tags.
* `tests/` - Tests of the system
    * `tests.rs` - Mod manifest; no other code.
    * `testing.rs` - Helpers such as lazy singletons and setup/teardown functions.
    * `command/` - Command tests that run the app with various args.
    * `markdown/` - Markdown tests, such as input parsing, including front matter kinds.
    * `render/` - Render tests for converting input markdown to output HTML.
    * `src/` - Source tests for extras, such as data, files, and functions.
    * `tutorial/` - Tutorial tests that match the README documentation
