# Documentation

* [crates](crates)
* [errors](errors)
* [files](files)
* [logs](logs)
* [matter](matter)
* [state](matter)
* [testing](testing)
* [templating](templating)
* [comparisons](comparisons)


## Implementation overview

Implementation broadly has these steps:

* The app launches and reads its configuration file; the implementation uses the `confy` crate.

* The app parses its command line options; the implementation uses the `clap` crate.

* The app creates a `config` struct that is used for all the rest of the work.

* The app iterates on any input files; the implementation is a typical run loop.

For each file:

* The app reads the file markdown text.

* The `MatterParser` code scans the text to discover any front matter text, such as with a HTML comment, JSON object, TOML configuration, or YAML document. If found, the code parses the front matter text to a `State` struct.

* The app converts the Markdown content text to HTML, and inserts it into the state, in order to make the HTML available for the next steps.

* The `Templater` code merges the HTML and the state, doing the variable substitution using the Tera templating engine or the Handlebars templating engine.

* The app saves the result as a corresponding HTML file.
