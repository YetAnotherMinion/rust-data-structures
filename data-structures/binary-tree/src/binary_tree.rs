use std::iter::IntoIterator;
/*
enum BinaryTree<'a, T> {
    Node{val: T: 'a, left: &'a BinaryTree<'a, T>, right: &'a BinaryTree<'a, T>},
    Leaf{val: T: 'a}
}
*/
#[derive(Debug, PartialEq, Clone)]
enum BinaryTree<T> {
    Node{value: T, left: Box<BinaryTree<T>>, right: Box<BinaryTree<T>>},
    Leaf{value: T},
}

impl<T> BinaryTree<T> {
    fn new_node(value: T, left: BinaryTree<T>, right: BinaryTree<T>) -> BinaryTree<T> {
        BinaryTree::Node{value: value, left: Box::new(left), right: Box::new(right)}
    }
    
    fn new_leaf(element: T) -> BinaryTree<T> {
        BinaryTree::Leaf{value: element}
    }
}

impl<T> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = BinaryTreeIterator<T>;
        
    fn into_iter(self) -> BinaryTreeIterator<T> {
        BinaryTreeIterator::new(self)
    }
}

struct BinaryTreeIterator<T> {
    right_nodes:  Vec<BinaryTree<T>>,
    current_node: Option<T>,
}

impl<T> BinaryTreeIterator<T> {
    fn new(node: BinaryTree<T>) -> BinaryTreeIterator<T> {
        let mut iter = BinaryTreeIterator{ right_nodes: vec![], current_node: None };
        iter.add_left_subtree(node);
        iter
    }

    fn add_left_subtree(&mut self, mut node: BinaryTree<T>) {
        loop {
            match node {
                BinaryTree::Node{value, left, right} => {
                    self.right_nodes.push(*right);
                    node = *left
                },
                BinaryTree::Leaf{value} => {
                    self.current_node = Some(value);
                    break;
                }
            }
        }
    }
}

impl<T> Iterator for BinaryTreeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // Get the item we're going to return
        let result = self.current_node.take();

        if let Some(node) = self.right_nodes.pop() {
            self.add_left_subtree(node);
        }
        result
    }
}

mod test {
    use super::BinaryTree;

    #[test]
    fn creation() {
        BinaryTree::new_leaf(10);

        let root = BinaryTree::new_node(40, BinaryTree::new_leaf(37), BinaryTree::new_leaf(45));
        BinaryTree::new_node(50, root.clone(), BinaryTree::new_leaf(60));
        
            
        assert_eq!(root, root.clone());
    }

    #[test]
    fn btree_iteration() {
        assert_eq!(BinaryTree::new_leaf(123).into_iter().collect::<Vec<_>>(), vec![123]);
        let btree = BinaryTree::new_node(100, BinaryTree::new_leaf(4), BinaryTree::new_node(120, BinaryTree::new_leaf(110), BinaryTree::new_leaf(130)));
        assert_eq!(btree.into_iter().collect::<Vec<_>>(), vec![4,110,130]);
    }

    #[test]
    fn deffered_construction() {
        let 
        for x in 0..10 {
            
        }
    }
}






