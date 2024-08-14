# AsRef<str> versus Into<String>

We prefer to use function definitions with flexible strings:

* `AsRef<str>` for a function parameter that uses borrowing, such as a readable `&str`.

* `Into<String>` for a function parameter that takes ownership, such as writable `String`.


## Example of AsRef<str>

Before refactoring:

```rust
fn foo(s: &str)
```

After refactoring:

```rust
fn foo(s: impl AsRef<str>)
```


## Example of Into<String>

Before refactoring:

```rust
fn foo(s: String)
```

After refactoring:

```rust
fn foo(s: impl Into<String>)
```
