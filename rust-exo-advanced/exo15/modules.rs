mod book_mod {
    pub struct Book {
        title: String,
    }

    impl Book {
        pub fn new(title: &str) -> Book {
            Book {
                title: title.to_string(),
            }
        }
    }

    pub fn read_book(book: &Book) {
        println!("Reading book: {}", book.title);
    }
}

fn main() {
    let my_awesome_book = book_mod::Book::new("My Incredible Book");

    book_mod::read_book(&my_awesome_book);
}