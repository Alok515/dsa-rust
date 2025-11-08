
mod list {
    use std::ops::Drop;
    struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    pub struct List<T> {
        head: Option<Box<Node<T>>>,
    }

    impl<T> List<T> {
        pub fn new() -> List<T> {
            List { head: None }
        }

        pub fn push(&mut self, value: T) {
            let new_node = Box::new(Node {
                value: value,
                next: self.head.take()
            });

            self.head = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|current_node| {
                self.head = current_node.next;

                current_node.value
            })
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut current_node = self.head.take();

            println!("List Dropped");
            while let Some(mut node) = current_node {
                current_node = node.next.take();
            }
        }
    }
}

fn main() {
    use list::List;
    let mut list_i32 = List::new();

    list_i32.push(2);
    list_i32.push(3);
    
    println!("List: {:?}", list_i32.pop());
    println!("List: {:?}", list_i32.pop());

    let mut list_string = List::new();

    list_string.push(String::from("Hello"));
    list_string.push(String::from("World"));

    println!("List: {:?}", list_string.pop());
    println!("List: {:?}", list_string.pop());
}