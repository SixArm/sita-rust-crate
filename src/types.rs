////
// Semantic collections
////

/// `Map` is our typical map collection and is an ordered map.
///
/// This is implemented in Rust by using `BTreeMap`.
///
/// Example:
///
/// ```
/// let my_map: Map<String, String> = Map::new();
/// ```
///
#[allow(dead_code)] pub type Map<K,V> = ::std::collections::BTreeMap<K,V>;

/// Create a typical map collection with elements.
///
/// Example:
///
/// ```
/// let x: Map<String, String> = map!(
///     1 => 2,
///     3 => 4,
/// );
/// assert_eq!(x[1], 2);
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! map { 
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                m.insert($k, $v);
            )*
            m
        }
    };
}

/// `Set` is our typical set collection and is an ordered set.
///
/// This is implemented in Rust by using `BTreeSet`.
///
/// Example:
///
/// ```
/// let my_set: Set<String> = Set::new();
/// ```
///
#[allow(dead_code)] pub type Set<T> = ::std::collections::BTreeSet<T>;

/// Create a typical set collection with elements.
///
/// Example:
///
/// ```
/// let x: Set<String> = set!(1, 2);
/// assert!(x.contains(&1));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut m = ::std::collections::BTreeSet::new();
            $(
                m.insert($x);
            )*
            m
        }
    };
}

/// `Stack` is our typical stack collection.
///
/// This is implemented in Rust by using `Vec`.
///
/// Example:
///
/// ```
/// let my_stack: Stack<String> = Stack::new();
/// ```
///
#[allow(dead_code)] pub type Stack<T> = ::std::vec::Vec<T>;

/// Create a typical stack collection with elements.
///
/// Example:
///
/// ```
/// let x: Stack<String> = stack!(1, 2);
/// assert!(x.contains(&1));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! stack {
    ( $( $x:expr ),* ) => {
        {
            let mut m = ::std::vec::Vec::new();
            $(
                m.push($x);
            )*
            m
        }
    };
}

/// `Queue` is our typical queue collection and is a double-ended queue.
///
/// This is implemented in Rust by using `VecDeque`.
///
/// Example:
///
/// ```
/// let my_queue: Queue<String> = Queue::new();
/// ```
///
#[allow(dead_code)] pub type Queue<T> = ::std::collections::VecDeque<T>;

/// Create a typical queue collection with elements.
///
/// Example:
///
/// ```
/// let x: Queue<String> = queue!(1, 2);
/// assert!(x.contains(&1));
/// ```
///
/// OPTIMIZE: add pre-allocation logic.
///
#[allow(unused_macros)]
macro_rules! queue {
    ( $( $x:expr ),* ) => {
        {
            let mut m = ::std::collections::VecDeque::new();
            $(
                m.push_back($x);
            )*
            m
        }
    };
}

////
// Semantic strings
////


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

/// HTML `str` typically for Hyper Text Markup Language.
///
/// Example:
///
/// ```
/// let s: HtmlStr = "<div>foo</div>");
/// ```
///
#[allow(dead_code)] pub type HtmlStr = str;

/// HTML `String` typically for Hyper Text Markup Language.
///
/// Example:
///
/// ```
/// let s: HtmlString = HtmlString::from("<div>foo</div>");
/// ```
///
#[allow(dead_code)] pub type HtmlString = String;

/// JSON `str` typically for JavaScript Object Notation.
///
/// Example:
///
/// ```
/// let s: JsonStr = "{ \"alpha\": \"bravo\" }";
/// ```
///
#[allow(dead_code)] pub type JsonStr = str;

/// JSON `String` typically for JavaScript Object Notation.
///
/// Example:
///
/// ```
/// let s: JsonString = JsonString::from("{ \"alpha\": \"bravo\" }");
/// ```
///
#[allow(dead_code)] pub type JsonString = String;

/// Markdown `str` typically for Markdown text.
///
/// Example:
///
/// ```
/// let s: MarkdownStr = "**foo**";
/// ```
///
#[allow(dead_code)] pub type MarkdownStr = str;

/// Markdown `String` typically for Markdown text.
///
/// Example:
///
/// ```
/// let s: MarkdownString = MarkdownString::from("**foo**");
/// ```
///
#[allow(dead_code)] pub type MarkdownString = String;

/// TOML `str` typically for Tom's Obvious Minimal Language.
///
/// Example:
///
/// ```
/// let s: TomlStr = "alpha = \"bravo\"";
/// ```
///
#[allow(dead_code)] pub type TomlStr = str;

/// TOML `String` typically for Tom's Obvious Minimal Language.
///
/// Example:
///
/// ```
/// let s: TomlString = TomlString::from("alpha = \"bravo\"");
/// ```
///
#[allow(dead_code)] pub type TomlString = String;

/// URL `str` typically for Uniform Resource Locator.
///
/// Example:
///
/// ```
/// let s: UrlStr = "https://example.com";
/// ```
///
#[allow(dead_code)] pub type UrlStr = str;

/// URL `String` typically for Uniform Resource Locator.
///
/// Example:
///
/// ```
/// let s: UrlString = UrlString::from("https://example.com");
/// ```
///
#[allow(dead_code)] pub type UrlString = String;

/// YAML `str` typically for Yet Another Markup Language.
///
/// Example:
///
/// ```
/// let s: YamlStr = "alpha: \"bravo\"";
/// ```
///
#[allow(dead_code)] pub type YamlStr = str;

/// YAML `String` typically for Yet Another Markup Language.
///
/// Example:
///
/// ```
/// let s: YamlString = YamlString::from("alpha: \"bravo\"");
/// ```
///
#[allow(dead_code)] pub type YamlString = String;

#[cfg(test)]
mod tests {

    #[test]
    fn test_map_macro_with_oneline() {
        let x = map!(1 => 2, 3 => 4);
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    #[test]
    fn test_map_macro_with_multiline() {
        let x = map!(
            1 => 2, 
            3 => 4
        );
        assert_eq!(x.get(&1).unwrap(), &2);
    }

    //TODO
    // #[test]
    // fn test_map_macro_with_empty() {
    //     let x: map!();
    //     assert_eq!(x.is_empty());
    // }

    #[test]
    fn test_queue_macro_with_oneline() {
        let x = queue!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_queue_macro_with_multiline() {
        let x = queue!(
            1, 
            2
        );
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_queue_macro_with_empty() {
    //     let x: queue!();
    //     assert_eq!(x.is_empty());
    // }

    #[test]
    fn test_set_macro_oneline() {
        let x = set!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_set_macro_multiline() {
        let x = set!{
            1,
            2
        };
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_set_macro_with_empty() {
    //     let x: set!();
    //     assert_eq!(x.is_empty());
    // }

    #[test]
    fn test_stack_macro_with_oneline() {
        let x = stack!(1, 2);
        assert!(x.contains(&1));
    }

    #[test]
    fn test_stack_macro_with_multiline() {
        let x = stack!(
            1, 
            2
        );
        assert!(x.contains(&1));
    }

    //TODO
    // #[test]
    // fn test_stack_macro_with_empty() {
    //     let x: stack!();
    //     assert_eq!(x.is_empty());
    // }

}