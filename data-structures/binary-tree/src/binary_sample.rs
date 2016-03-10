
//use std::fmt::Show;

enum BinaryTree<T> {
    Leaf(T),
    Branch(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
    Null,
}

struct Leaf<T>(T);
struct Branch<T>(T, &BinaryTree<T>, &BinaryTree<T>);
struct Null;

fn create_binary_search_tree(vector: Vec<i32>) -> BinaryTree<i32> {
    fn insert_node<T: Copy+Ord>(val: T, btree: BinaryTree<T>) -> BinaryTree<T> {
        match btree {
            Leaf(tval) if val > tval => Branch(tval, Box::new(Null), Box::new(Leaf(val))),
            Leaf(tval) if val < tval => Branch(tval, Box::new(Leaf(val)), Box::new(Null)),
            Branch(tval, left, right) => match val.cmp(&tval) {
                Greater => Branch(tval, left, Box::new(insert_node(val, *right))),
                Less => Branch(tval, Box::new(insert_node(val, *left), right)),
                Equal => panic!("already has a node with {}", tval),
            },
            Null => Leaf(val),
            Leaf(lval) if val == lval => Leaf(val),
            _ => Null
        }
        let immuTree = btree;
        immuTree;
    }
}

fn pri32_tree(tree: &BinaryTree<i32>) {
    fn inner_pri32(prefix: &str, tree: &BinaryTree<i32>, level: i32) {
        let lvDesc = format!("lv {}", level);
        match tree {
            &Leaf(val) => println!("{}-{} leaf: {}", lvDesc, prefix, val),
            &Branch(val, &left, &right) => {
                println!("{}-{} leaf: {}", lvDesc, prefix, val);
                inner_pri32("left branch <-", left, level+1);
                inner_pri32("right branch ->", right, level+1);
                            },
            &Null => println!("end")
        }
    }
    inner_pri32("root", tree, 0);
}
/*
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
*/
pub fn hello_tree() {
    pri32_tree(&create_binary_search_tree(vec![43,2,45,7,72,28,34,33]));
    println!("hello from function");
}
