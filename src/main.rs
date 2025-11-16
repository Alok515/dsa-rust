
mod bst{
    use std::{cmp::{Ord, Ordering}};

    pub struct Node<T: Ord>{
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>
    }

    pub struct BinarySearchTree<T: Ord>{
        root: Option<Box<Node<T>>>
    }

    impl<T: Ord> BinarySearchTree<T> {
        pub fn new() -> Self {
            Self {
                root: None
            }
        }

        pub fn insert(&mut self, value: T) {
            Self::insert_recursive(&mut self.root, value);
        }

        pub fn search(&self, value: T) -> bool {
            let mut current = &self.root;

            while let Some(node) = current {
                match value.cmp(&node.value) {
                    Ordering::Less => current = &node.left,
                    Ordering::Greater => current = &node.right,
                    Ordering::Equal => return true
                }
            }
            false
        }

        pub fn in_order<'a>(&'a self) -> Vec<&'a T> {
            let mut res = Vec::new();
            Self::in_order_recursive(&self.root, &mut res);
            res
        }

        fn in_order_recursive<'a>(node: &'a Option<Box<Node<T>>>, res: &mut Vec<&'a T>) {
            if let Some(current_node) = node {
                //left 
                Self::in_order_recursive(&current_node.left, res);

                //root
                res.push(&current_node.value);

                //right
                Self::in_order_recursive(&current_node.right, res);
            }
        }

        fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) {
            match node {
                Some(current_node) => {
                    if value < current_node.value {
                        Self::insert_recursive(&mut current_node.left, value);
                    } else if value > current_node.value {
                        Self::insert_recursive(&mut current_node.right, value);
                    }
                }
                None => {
                    let new_node = Box::new(Node {
                        value: value,
                        left: None,
                        right: None
                    });
                    *node = Some(new_node);
                }
            }
        }
    }
}

fn main(){
    use bst::BinarySearchTree;
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    println!("Created Binary Search Tree");
    tree.insert(1);
    tree.insert(2);
    tree.insert(13);
    tree.insert(8);
    tree.insert(5);
    tree.insert(9);
    tree.insert(10);

    println!("Inserted Items in Tree");

    println!("Tree contains 1: {}", tree.search(1));
    println!("Tree contains 2: {}", tree.search(2));
    println!("Tree contains 13: {}", tree.search(130));
    println!("Tree contains 8: {}", tree.search(8));
    println!("Tree contains 5: {}", tree.search(5));
    println!("Tree contains 9: {}", tree.search(19));

    println!("----In-Order Traversal----");
    let in_order = tree.in_order();
    println!("{:?}", in_order);
}