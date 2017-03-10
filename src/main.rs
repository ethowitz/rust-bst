use std::cmp;
use std::fmt;

extern crate odds;
use odds::Fix;
use odds::fix;

struct Node<T> {
    key: i32,
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/*
impl fmt::Display for Node<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.key, &self.value)
    }
}
*/

impl<T: fmt::Display> Node<T> {
    fn in_order(&self) {
        if let Some(ref node) = self.left { node.in_order() };
        println!("{{key: {}, value: {}}}", self.key, self.value);
        if let Some(ref node) = self.right { node.in_order() };
    }
}

impl<T> Node<T> {
    // TODO: apparently this recursion is not idiomatic?
    pub fn insert(&mut self, key: i32, val: T) {
        let target = if key > self.key { &mut self.right } else { &mut self.left };
        match *target {
            None => *target = Some(Box::new(Node {
                            key: key, value: val, left: None, right: None})),
            Some(ref mut target) => target.insert(key, val)
        }
    }

    fn search_ref(&mut self, key: i32) -> Option<&mut Box<Node<T>>> {
        let search = | node: &mut Node<T> | -> Option<&mut Box<Node<T>>> {
            match key.cmp(&node.key) {
                cmp::Ordering::Less => search(node.left),
                cmp::Ordering::Equal => Some(node),
                cmp::Ordering::Greater => search(node.right)
            }
        };

        search(self)
    }

    pub fn delete(&mut self, key: i32) {
        // TODO: need to get mutable reference to parent's reference to this node
        if let Some(ref mut node) = self.search_ref(key) {
            match (node.left, node.right) {
                (None, None) => /* just set reference to this node to None */ true,
                (Some(c), None) | (None, Some(c)) => true,
                (Some(l), Some(r)) => true
            }
        } else {
            false
        };
    }

    pub fn compute_height(&self) -> u32 {
        let left_height =
            if let &Some(ref node) = &self.left {
                node.compute_height()
            } else { 0 };

        let right_height =
            if let &Some(ref node) = &self.right {
                node.compute_height()
            } else { 0 };

        cmp::max(left_height, right_height) + 1
    }
}

fn main() {
    // TODO: this is obviously clunky and exposes structure of tree/map and nodes
    let mut root = Box::new(Node::<i32> { key: 5, value: 5, left: None, right: None });
    root.insert(2, 2);
    root.insert(9, 9);
    root.insert(-1, -1);
    root.insert(12, 12);
    root.insert(11, 11);
    root.insert(-5, -5);
    root.in_order();
}
