use crate::graphql::types::Book;
use crate::graphql::types::input::NewBook;
use crate::repository;

impl From<repository::schema::Book> for Book {
    fn from(p: repository::schema::Book) -> Self {
        Self {
            id: match p.id {
                Some(id) => id.to_string(),
                None => "".to_string(),
            },
            title: p.title,
            author: p.author,
        }
    }
}

impl From<NewBook> for repository::schema::Book {
    fn from(new_book: NewBook) -> Self {
        repository::schema::Book {
            id: None,
            title: new_book.title,
            author: new_book.author,
        }
    }
}
