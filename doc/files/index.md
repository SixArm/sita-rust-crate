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
    * `fun/` - Functions, including utilties, helpers, converters, etc.
    * `markdown/` - Mardown-related
        * `matter/` - Markdown front matter and back matter files
            * `state.rs` - State that holds front matter as a kind.
            * `kinds/` - Kinds of front matter
                * `html.rs` - Hyper Text Markup Language
                * `json.rs` - JavaScript Object Notation
                * `toml.rs` - Tom's Obvious Minimal Language
                * `yaml.rs` - Yet Another Markup Language
    * `templating/` - Templating-related
        * `kinds/` - Kinds of templating engines
            * `handlebars.rs` - Handlebars templating
            * `tera.rs` - Tera templating
* `tests/` - Tests of the system
    * `command/` - Command tests that run the app with various args
    * `function/` - Function tests, such as function example data and example files.
    * `markdown/` - Markdown tests, such as parsing, including front matter kinds
    * `tutorial/` - Tutorial tests that match the README documentation
