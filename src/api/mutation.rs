use crate::api::schema::{Book, NewBook};

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_todo(new_todo: NewBook) -> Book {
        Book {
            id: "hhgfherr6".to_string(),
            title: new_todo.title,
            author: new_todo.author,
        }
    }
}
