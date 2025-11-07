
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
    }
}

fn main() {
    use list::List;
    let my_list = List::new();
}