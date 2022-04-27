# README

This directory is for source code unit tests that need test extras, such as test data files.

This directory is an organizational pattern that aligns the source code path `./src` and tests path `./tests/src`

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
