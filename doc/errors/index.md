# Errors using thiserror

For errors we use the crate `thiserror` because it is well-maintained, pragmatic, and provides detailed errors suitable for libraries. 

We removed the crate `error-chain` because it wasn't well-maintained.


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

* The error message starts with the enum variant name and a Unicode right arrow.

* The error message for a struct includes each field and its debug representation, so long as it's practical, i.e. we want to behave akin to a library error, and we want print as much information as possible for developers who are debugging.

* For wrapping errors, prefer using the simple form.


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
    env_logger::init();
    match crate::app::run::run() {
        Ok(()) => {
            std::process::exit(0);
        }
        Err(err) => {
            error!("{:?}", err);
            std::process::exit(1);
        }
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


## Wrap errors

Say you have a file `alfa.rs` with any typical function that can return an error such as:

```rust
pub fn positive(x: i8) -> Result<(), Error> {
    if x > 0 {
        return x
    } else {
        Error::Parma(x)
    }
}
    
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Parma ➡ {0:?}")]
    Param(i8),
}
```

Say you also have a file `bravo.rs` that calls `alfa.rs` such as:

```rust
pub fn f(x: i8) -> Result<(), Error> {
    crate::alfa::positive(3)
    .map_or_else(
        |err| Error::Wrap(err),
        |val| val.ok()
    )?
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Wrap ➡ {0:?}")]
    Wrap(crate::alfa::Error)
}
```
