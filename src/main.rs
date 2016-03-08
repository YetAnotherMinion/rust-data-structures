extern crate rust_data_structures;
use rust_data_structures::graph;

trait Printable {
    fn printme(&self);
}


struct Node {
    x: i32,
}

impl Printable for Node {
    fn printme(&self) {
        println!("This is x {}", self.x);
    }
}

struct Edge {
    y: i32,
}

impl Printable for Edge {
    fn printme(&self) {
        println!("this is from an edge {}", self.y);
    }
}


fn main() {
    println!("Hello from giving up figuring out crates today on livestream");
    let a = Node {
        x: 7,
    };
    let b = Edge {
        y: (1<<32 - 1),
    };
    a.printme();
    b.printme();
    
    let c = graph::Node {
        x: 1,
        y: 2,
    };
    
}

