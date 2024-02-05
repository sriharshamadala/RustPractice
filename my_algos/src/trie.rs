// Our tries only contains chars from 'a'..'z'
const SIZE: usize = 26;

// ascii value of 'a'
const ASCII_OFFSET: usize = 97;

fn to_index(ch: char) -> usize {
    (ch as usize) - ASCII_OFFSET
}

#[derive(Default)]
struct Trie {
    links: [Option<Box<Trie>>; SIZE],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }
    
    fn insert(&mut self, word: String) {
        let mut node = self;
        for index in word.chars().map(to_index) {
            node = node.links[index].get_or_insert(Box::new(Trie::new()));
        }
        node.is_end = true;
    }
    
    /// Traverses the trie and returns the end node if the whole word exists
    /// Otherwise, we return None
    fn end_node(&self, word: &String) -> Option<&Trie> {
        let mut node = self;
        for index in word.chars().map(to_index) {
            match node.links[index] {
                None => { return None; },
                Some(ref child) => { node = child }
            }
        }

        Some(node)
    }

    fn search(&self, word: String) -> bool {
        if let Some(node) = self.end_node(&word) {
            node.is_end
        }
        else {
            false
        }
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.end_node(&prefix).is_some()
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let trie = Trie::new();
        assert!(!trie.search("fizz".to_string()));
        assert!(!trie.starts_with("fizz".to_string()));
    }

    #[test]
    fn simple_trie() {
        let mut trie = Trie::new();

        trie.insert("buzz".to_string());
        trie.insert("fizzbuzz".to_string());

        assert!(trie.search("fizzbuzz".to_string()));
        assert!(trie.search("buzz".to_string()));
        assert!(trie.starts_with("fizz".to_string()));
        assert!(!trie.search("fizz".to_string()));
    }
}