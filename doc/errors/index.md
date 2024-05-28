# Errors using thiserror

For errors we use the crate `thiserror` because it is well-maintained, pragmatic, and provides detailed errors suitable for libraries. We removed the crate `error-chain` because it wasn't well-maintained.


## pub enum Error

The pattern for error definition looks like this:

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {

    #[error("AlfaBravo ➡ {0:?}")]
    AlfaBravo(std::io::Error),

    #[error("CharlieDelta ➡ echo: {echo:?}, foxtrot: {foxtrot:?}")]
    CharlieDelta {
        echo: String,
        foxtrot: String,
    }

}
```

### Our convention

Our convention for `pub enum Error`:

* The enum is named `Error`, not anything else. A side effect of this convention: in each file, the file's enum `Error` convention supersedes any other `Error` such as `std::error::Error`, i.e. in each file, any other `Error` must be fully qualified.

* The error message starts with the enum variant and a Unicode right arrow.

* The error message for a struct includes each field and its debug representation, so long as it's practical, i.e. we want to behave akin to a library error, and we want print as much information as possible for developers who are debugging.


## Catch errors

Our preferred pattern for handling a `Result` uses `map_or_else` to return the error `err` or unwrap the value `val` like this:

```rust
foo().map_or_else(
    |err| MyError::StdIoError(err),
    |val| val.ok()
)?;
```


### Catch errors in main

Our convention is a file `main.rs` where the setup happens:

```rust
fn main() {
    if let Err(ref e) = crate::run::run() {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        std::process::exit(1);
    }
}
```

Our convention is a file `run.rs` where the work happens:

```rust
pub(crate) fn run() -> Result<()> {
    …
    Ok(())
}
```
