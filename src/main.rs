use std::ops::Drop;

mod list {
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    pub struct List {
        head: Option<Box<Node>>,
    }

    impl List {
        pub fn new() -> List {
            List { head: None }
        }

        pub fn push(&mut self, value: i32) {
            let new_node = Box::new(Node {
                value: value,
                next: self.head.take()
            });

            self.head = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<i32> {
            self.head.take().map(|current_node| {
                self.head = current_node.next;

                current_node.value
            })
        }
    }

    impl Drop for List {
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
    let mut my_list = List::new();
    my_list.push(3);
    my_list.push(6);
    my_list.push(9);

    println!("List Created");
    println!("Popped: {:?}", my_list.pop());
    println!("Popped: {:?}", my_list.pop());
    println!("Popped: {:?}", my_list.pop());
    println!("Popped: {:?}", my_list.pop());

    my_list.push(4);
    my_list.push(7);
    my_list.push(10);

    println!("End of main, list is about to be dropped.");
}