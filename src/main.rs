use self::models::*;
use diesel::prelude::*;
use rust_book::*;

fn main() {
    use self::schema::rust_book_table::dsl::*;

    let connection = &mut establish_connection();
    let results = rust_book_table
        .limit(10)
        .select(Book::as_select())
        .load(connection)
        .expect("Error loading books");

    println!("Displaying {} books", results.len());
}
