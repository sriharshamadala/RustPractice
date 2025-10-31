//! My implementation of a doubly-linked list which supports the following API:
//!   - new
//!   - add_front
//!   - add_back
//!   - add_front_node
//!   - add_back_node
//!   - remove
//!   - move_head
//!   - move_tail

use std::rc::Rc;
use std::cell::RefCell;

// Using Rc<RefCell> instead of Box to allow for multiple ownership
// Needed to implement get_head()
type NodePtr = Rc<RefCell<DllNode>>;
type NodeOpt = Option<NodePtr>;

#[derive(Default)]
struct DllNode {
    val: i32,
    prev: NodeOpt,
    next: NodeOpt
}

impl DllNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            prev: None,
            next: None
        }
    }
}

#[derive(Default)]
struct DllList {
    head: NodeOpt,
    tail: NodeOpt
}

impl DllList {
    pub fn new() -> Self {
        Default::default()
    }

    fn get_head(&self) -> NodeOpt {
        self.head.as_ref().map(|node_ptr| Rc::clone(node_ptr))
    }

    fn get_tail(&self) -> NodeOpt {
        self.tail.as_ref().map(|node_ptr| Rc::clone(node_ptr))
    }

    // Create a node with specified val and add as head
    pub fn add_front(&mut self, val: i32) {

    }

}