# README

This directory is a pattern that emphasizes alignment of the source path and the tests path.

This directory is for source unit tests that need test extras, such as test data files.

For this file:

```sh
src/mymodule/myfile.rs
```

Put any test extras here:

```sh
tests/src/mymodule/myfile/
```

For this function:

```rust
fn myfunction()
```

Put any test extras here:

```sh
tests/src/mymodule/myfile/myfunction/
```
