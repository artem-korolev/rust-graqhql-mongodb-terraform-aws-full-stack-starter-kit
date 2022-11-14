use juniper::GraphQLObject;

use crate::repository;

#[derive(GraphQLObject)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct NewBook {
    pub title: String,
    pub author: String,
}


impl From<repository::schema::Book> for Book {
    fn from(p: repository::schema::Book) -> Self {
        Self {
            id: match p.id {Some(id) => id.to_string(), None => "".to_string()},
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
