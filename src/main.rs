
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
    }
}

fn main() {
    use list::List;
    let mut my_list = List::new();
    my_list.push(3);
    my_list.push(6);
    my_list.push(9);

    println!("List Created");
}