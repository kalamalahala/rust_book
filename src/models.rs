use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::rust_book_table)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published_date: String,
    pub isbn: String,
    pub page_count: i32,
    pub excerpt: String,
    pub synopsis: String,
    pub cover_image: String,
    pub created_at: String,
    pub updated_at: String
}