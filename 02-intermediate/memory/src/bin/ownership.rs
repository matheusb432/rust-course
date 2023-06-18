struct Book {
    pages: i32,
    rating: f32,
}

// NOTE setting type to &Book, a reference of Book to borrow
fn display_pages(book: &Book) {
    println!("Pages: {}", book.pages);
}

fn display_rating(book: &Book) {
    println!("Rating: {}", book.rating);
}

fn main() {
    let book = Book {
        pages: 50,
        rating: 9.5,
    };

    // NOTE ownership of book will move to display_pages()
    // display_pages(book);
    // NOTE will not work since book has already been moved and deleted in memory inside `display_pages`
    // display_rating(book);

    display_pages(&book);
    // NOTE will work, the main() still owns `book` and it has not been deleted yet
    display_rating(&book);
}
