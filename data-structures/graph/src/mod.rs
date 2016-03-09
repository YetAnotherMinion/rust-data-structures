use self::node::Node;
use self::edge::Edge;

pub mod node;
pub mod edge;

fn main() {
    let a = Node {
        x: 3,
    };
    println!("hello from graph package");
}

