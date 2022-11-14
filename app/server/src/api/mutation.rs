use bson::doc;

use crate::{
    api::schema::{Book, NewBook},
    repository::{self, Repository},
};

pub struct MutationRoot;

#[juniper::graphql_object(context = Repository)]
impl MutationRoot {
    async fn create_book(context: &Repository, new_book: NewBook) -> Book {
        let insert_result = context
            .get_books_collection()
            .insert_one(repository::schema::Book::from(new_book), None)
            .await
            .unwrap();
        let new_book = context
            .get_books_collection()
            .find_one(Some(doc! {"_id": insert_result.inserted_id}), None)
            .await
            .unwrap()
            .unwrap();
        new_book.into()
    }
}
