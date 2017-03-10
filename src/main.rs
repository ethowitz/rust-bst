use std::cmp;
use std::fmt;

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
    fn insert(&mut self, key: i32, val: T) {
        let target = if key > self.key { &mut self.right } else { &mut self.left };
        match *target {
            None => *target = Some(Box::new(Node {
                            key: key, value: val, left: None, right: None})),
            Some(ref mut target) => target.insert(key, val)
        }
    }

    // fn delete(&mut self, val: i32) {
    //
    // }

    fn compute_height(&self) -> u32 {
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
    // TODO: add code to randomly generate nodes
    root.insert(2, 2);
    root.insert(9, 9);
    root.insert(-1, -1);
    root.insert(12, 12);
    root.insert(11, 11);
    root.insert(-5, -5);
    root.in_order();
}
