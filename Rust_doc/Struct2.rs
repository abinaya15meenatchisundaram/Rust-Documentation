// Define the struct
struct Book {
    title: String,
    author: String,
    year_of_publication: u32,
}

// Write a function to display the details of the book
fn display_book_details(book: Book) {
    println!("Title: {}", book.title);
    println!("Author: {}", book.author);
    println!("Year of Publication: {}", book.year_of_publication);
}

fn main() {
    // Create an instance of Book and call display_book_details function
    let book1 = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        year_of_publication: 2018,
    };
    display_book_details(book1);
}