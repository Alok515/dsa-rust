
mod library {
    pub mod fiction {
        pub struct Book {
            pub title: String,
            pub author: String
        }

        pub fn borrow_book() {
            println!("Borrowing a Fiction Book...");
        } 
    }
    pub mod non_fiction {
        pub fn check_out_book() {
            println!("Checking out a Non Fiction Book...");
        }
    }
}

fn main () {
    use library::fiction::Book;
    use library::{fiction, non_fiction};

    let book1 = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald")
    };

    fiction::borrow_book();
    non_fiction::check_out_book();
}