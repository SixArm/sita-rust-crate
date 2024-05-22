/// Pathable `String` typically for Unix system path or glob.
///
/// Example:
///
/// ```
/// let s: PathableString = PathableString::from("foo/**/*");
/// ```
///
#[allow(dead_code)] pub type PathableString = String;

/// TOML `String` typically for Tom's Obvious Minimal Language.
///
/// Example:
///
/// ```
/// let s: TomlString = TomlString::from("alpha = \"bravo\"");
/// ```
///
#[allow(dead_code)] pub type TomlString = String;
