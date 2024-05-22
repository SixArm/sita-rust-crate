/// Pathable `str` typically for Unix system path or glob.
///
/// Example:
///
/// ```
/// let s: PathableStr = "foo/**/*";
/// ```
///
#[allow(dead_code)] pub type PathableStr = str;

/// Pathable `String` typically for Unix system path or glob.
///
/// Example:
///
/// ```
/// let s: PathableString = PathableString::from("foo/**/*");
/// ```
///
#[allow(dead_code)] pub type PathableString = String;
