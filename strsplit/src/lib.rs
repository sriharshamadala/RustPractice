/// This module implements String splitting functionality.
/// Given a string and a delimiter, split the string into substrings
///

#[derive(Debug)]
struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// '_ is anonymous lifetime; We are asking the compiler to guess the lifetime
// This only works if there is one possible value.
impl<'a> StrSplit<'a> {
    // We could return StrSplit but edit in more places should the name change.
    // If this is a long function, we need to scroll to figure what type Self refers to.
    pub fn new (haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter
        }
    }
}

/*
fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c)).next().expect("StrSplit always returns something")
}
*/

// This allows us to do, "for part in StrSplit {}"
impl<'a> Iterator for StrSplit<'a> {
    // We return a str, but what is the expected lifetime of this?
    // It is obvious to us that this has remainder's lifetime but Rustc needs to know.
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // Some(ref mut r) - Take reference of the remainder when it matches Some()
        // Some(r) - Own remainder through copy.
        // Some(&mut r) - does matching when remainder is of type &mut T, which its not.
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                // take() - if option = None { return; } else 
                // {set Option to None;
                // return Some;}
                self.remainder.take()
            }
        } else {
            None
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

#[test]
fn empty_haystack() {
    let haystack = "";
    if let Some(result) = StrSplit::new(haystack, " ").next() {
        assert_eq!(result, "");
    } else {
        // Not supposed to happen.
        assert!(false);
    }
}
