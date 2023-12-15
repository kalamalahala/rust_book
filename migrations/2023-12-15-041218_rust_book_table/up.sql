-- Your SQL goes here SQLite3 syntax
CREATE TABLE rust_book_table (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    published_date TEXT NOT NULL,
    isbn TEXT NOT NULL,
    page_count INTEGER NOT NULL,
    excerpt TEXT NOT NULL,
    synopsis TEXT NOT NULL,
    cover_image TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
)