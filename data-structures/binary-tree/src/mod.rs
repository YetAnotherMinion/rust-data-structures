
pub mod binary_tree;

fn main() {
    let x = Box::new(3u32);
    println!("x: {}", x);
    let mut y = Box::new(*x);
    *y = 7;
    y = x;
    
    println!("y: {}", y);
    //println!("x: {}", x);
    println!("hello from binary tree");
}

