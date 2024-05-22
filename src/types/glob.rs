/// Glob `str` typically for Unix system path pattern matching.
///
/// Example:
///
/// ```
/// let s: GlobStr = "foo/**/*";
/// ```
///
#[allow(dead_code)] pub type GlobStr = str;

/// Glob `String` typically for Unix system path pattern matching.
///
/// Example:
///
/// ```
/// let s: GlobString = GlobString::from("foo/**/*");
/// ```
///
#[allow(dead_code)] pub type GlobString = String;
