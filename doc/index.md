# Documentation


## File organization

* `src/`
    * `main.rs` - the main function that loads everything then calls `run.rs`.
    * `run.rs` - the core business logic that runs everything; called by `main`.
    * `types.rs` - type aliases and related macros.
    * `util.rs` - utility functions
    * `app/` - application files typical of any of our apps
        * `args.rs` - arguments as a struct; can be created by `clap.rs`.
        * `clap.rs` - command line argument parsing, which creates an `args` struct.
        * `confy.rs` - configuration file parsing tests, which load configuration variables.
    * `markdown/` - Mardown-related
        * `matter/` - Markdown front matter and back matter files
            * `state.rs` - state that holds front matter as a kind.
            * `kinds/` - kinds of front matter
                * `html` - Hyper Text Markup Language
                * `json` - JavaScript Object Notation
                * `toml` - Tom's Obvious Minimal Language
                * `yaml` - Yet Another Markup Language
* `tests/` - system tests
    * `command/` - tests that run the command with various args
    * `function/` - each subdirectory relates to one function
    * `tutorial/` - tutorial files that match the README documentation
    * `markdown/` - tests of markdown parsing, including front matter kinds
