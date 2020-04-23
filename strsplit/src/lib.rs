/// This module implements String splitting functionality.
/// Given a string and a delimiter, split the string into substrings
///

struct StrSplit {
    remainder: &str,
    delimiter: &str,
}

impl StrSplit {
    // We could return -> StrSplit but we have to edit in more places should the name change.
    // If this is a long function, we may need to scroll to figure what type Self refers to.
    pub fn new (haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter
        }
    }
}

// This allows us to do, "for part in StrSplit {}"
impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut Self) -> Option<Self::Item> {
        if let Some(next_delim) = Self.remainder.find(Self.delimiter) {
            let until_delimiter = &Self.remainder[..next_delim];
            Self.remainder = &Self.remainder[(next_delim + Self.delimiter.len())..];
            Some(until_delimiter)
        } else if Self.remainder.is_empty() {
            None
        } else {
            // Why bother with emptying the remainder in the last step?
            // Easy to return Some(Self.remainder)
            let rest = Self.remainder;
            Self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn basic_test() {
    let haystack = "a b c d e";
    /*
    for letter in StrSplit::new(haystack, " ") {
        //a
        //b
        //...
    }
    */
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}

