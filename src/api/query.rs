use crate::{api::schema::Book, repository::Repository};
use futures::stream::TryStreamExt;

pub struct QueryRoot;

#[juniper::graphql_object(context = Repository)]
impl QueryRoot {
    async fn books(context: &Repository) -> Vec<Book> {
        context
            .get_books_collection()
            .find(None, None)
            .await
            .unwrap()
            .try_collect()
            .await
            .unwrap_or_else(|_| vec![])
            .into_iter().map(|x| x.into()).collect()
    }
}
