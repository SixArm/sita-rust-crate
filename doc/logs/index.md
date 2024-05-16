# Logs

Sita currently has some logging capabilities, and more to come.

Sita currently uses the environment variable `RUST_LOG` in order to set the log level.

Examples:

```sh
RUST_LOG=trace sita --input example.md
RUST_LOG=debug sita --input example.md
```

Sita has work in progress to enable verbose output via the command line option `--verbose`. Currently this sets the internal log level, yet doen't print anything.

Sita has work in progress to enable test output via the command line option `--test`. Currently this prints the Sita configuration.
