/// This module implements String splitting functionality.
/// Given a string and a delimiter, split the string into substrings
///

#[derive(Debug)]
struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

// '_ is anonymous lifetime; We are asking the compiler to guess the lifetime
// This only works if there is one possible value.
impl<'a> StrSplit<'a> {
    // We could return StrSplit but edit in more places should the name change.
    // If this is a long function, we need to scroll to figure what type Self refers to.
    pub fn new (haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter
        }
    }
}

// This allows us to do, "for part in StrSplit {}"
impl<'a> Iterator for StrSplit<'a> {
    // We return a str, but what is the expected lifetime of this?
    // It is obvious to us that this has remainder's lifetime but Rustc needs to know.
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            // Why bother with emptying the remainder in the last step?
            // Easy to return Some(Self.remainder)
            let rest = self.remainder;
            // Empty string has static lifetime.
            // But we said we will return something with lifetime 'a.
            // Reason this is ok is, so long as lifetime is >= its fine.
            self.remainder = "";
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
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}

