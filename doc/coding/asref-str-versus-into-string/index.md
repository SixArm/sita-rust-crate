# AsRef<str> versus AsRef<str>

We prefer to use function definitions with flexible strings:

* `AsRef<str>` for a function parameter that uses borrowing, such as a readable `&str`.

* `AsRef<str>` for a function parameter that takes ownership, such as writable `String`.

Example with concrete types:

```rust
fn foo(borrowable: &str, ownable: String) ...
```

Example with our preferred traits:

```rust
fn foo(borrowable: impl AsRef<str>, ownable: impl AsRef<str>) ...
```
