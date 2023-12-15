// @generated automatically by Diesel CLI.

diesel::table! {
    hello (id) {
        id -> Text,
    }
}

diesel::table! {
    rust_book_table (id) {
        id -> Integer,
        title -> Text,
        author -> Text,
        published_date -> Text,
        isbn -> Text,
        page_count -> Integer,
        excerpt -> Text,
        synopsis -> Text,
        cover_image -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    hello,
    rust_book_table,
);
