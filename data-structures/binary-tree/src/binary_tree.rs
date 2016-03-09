use std::cell::UnsafeCell;

struct Node1 {
    left: *Node,
    right: *Node,
}

struct Node2<'foo> {
    neighbors: UnsafeCell<Vec<&'foo Node<'foo>>>,
}

impl<'a> Node<'a> {
    fn new<'bar>
}

pub fn hello_tree() {
    println!("hello from function");
}
