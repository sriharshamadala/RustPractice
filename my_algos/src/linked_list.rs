struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Sized + Copy,
{
    pub fn new(value: T) -> Self {
        Node {
            val: value,
            next: None,
        }
    }

    pub fn get(&self) -> T {
        self.val
    }

    /* Not implementing From<Vec<T>> and instead choosing a method.
    Reasons based on the [discussion here](https://users.rust-lang.org/t/when-should-i-implement-from-instead-of-writing-a-method-that-returns-the-type/99751/2)
    impl<T> From<Vec<T>> for Node<T> {}
    */

    /*
    Attempt to iterate normally fails since head gets moved to prev_node initially
    and we cannot refer to head in the end
    pub fn from(v: &mut [T]) -> Option<Self> {
        if v.is_empty() {
            return None;
        }

        let mut head = Box::new(Node::new(v[0]));
        let mut prev_node = head;

        for value in v.into_iter() {
            let curr_node = Box::new(Node::new(*value));
            prev_node.next = Some(curr_node);
            prev_node = prev_node.next.unwrap();
        }

        Some(*head)
    }
    */

    // Iterating in reverse is easier
    pub fn from(v: &mut [T]) -> Option<Self> {
        if v.is_empty() {
            return None;
        }

        let mut v_iter = v.into_iter().rev();
        let tail = Box::new(Node::new(*v_iter.next().unwrap()));
        let mut prev_node = tail;

        for value in v_iter {
            let mut curr_node = Box::new(Node::new(*value));
            curr_node.next = Some(prev_node);
            prev_node = curr_node;
        }

        Some(*prev_node)
    }
}

/* TODO Need to implement iterator
struct NodeIterator<'a, T> {
    node_ref: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_ref) = self.node_ref {
            self.node_ref = (*node_ref).next;
            return Some(*node_ref);
        } else {
            return None;
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node::new(5);
        assert_eq!(5, node.get());
    }

    #[test]
    fn link_two_nodes() {
        let tail = Box::new(Node::new(43));
        let mut head = Box::new(Node::new(42));
        head.next = Some(tail);

        assert_eq!(43, head.next.unwrap().get());
    }

    #[test]
    fn create_from_array() {
        let mut input = [42, 43, 44];
        let head = Node::from(&mut input).unwrap();
        assert_eq!(42, head.get());
        let mut curr_node = head.next.unwrap();
        assert_eq!(43, curr_node.get());
        curr_node = curr_node.next.unwrap();
        assert_eq!(44, curr_node.get());
    }
}
