# Errors using error-chain


## Initialize errors

Add `error-chain` initiatization code as [recommended](https://brson.github.io/2016/11/30/starting-with-error-chain):

```rust
// Simple and robust error handling with error-chain!

// `error_chain!` can recurse deeply, so limit it.
#![recursion_limit = "1024"]

// Import the macro. Be sure to add `error-chain` in your `Cargo.toml`.
#[macro_use]
extern crate error_chain;

// We put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// that `error_chain!` creates.
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

pub use errors::*;
```


## Catch errors

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
