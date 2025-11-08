
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
    }
}

fn main(){
    use bst::BinarySearchTree;
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    println!("Created Binary Search Tree");
}