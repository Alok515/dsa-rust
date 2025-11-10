
mod bst{
    use std::cmp::Ord;

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
}