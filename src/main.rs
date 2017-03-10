use std::cmp;
use std::fmt;

struct Node<T> {
    key: i32,
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

/* TODO: trying to enable printing for when T = String, but not compiling...
impl fmt::Display for Node<String> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.key, self.value)
    }
}

impl<String> Node<String> {
    fn in_order(&self) {
        if let &Some(ref node) = &self.left { node.in_order() };
        print!("{{{} {}}} ", self.key, &self.value);
        if let &Some(ref node) = &self.right { node.in_order() };
    }
}
*/

// TODO: apparently this recursion is not idiomatic?
impl<T> Node<T> {
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
    let root = Box::new(Node::<String> { key: 5, value: "Ethan".to_string(), left: None, right: None });
    // root.insert(1);
    // root.insert(19);
    // root.insert(7);
    // root.insert(15);
    println!("{}", root.compute_height());
    // root.in_order();
}
