// #[macro_use]
// extern crate juniper;
mod repository;
mod server;

extern crate log;
extern crate pretty_env_logger;

use repository::get_books_collection;
use futures::stream::TryStreamExt;

#[tokio::main]
async fn main() {
    // // Init logger
    // pretty_env_logger::init();

    // // Start the API server to handle requests
    // server::start( ([127,0,0,1], 3000) ).await;

    test_database().await;
}


async fn test_database() {
    let book = repository::schema::Book {
        title: "The Grapes of Wrath".to_string(),
        author: "John Steinbeck".to_string(),
        id: None,
    };
    let repo = get_books_collection().await.unwrap();
    repo.insert_one(book, None).await.unwrap();

    let mut cursor = repo.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await.unwrap() {
        println!("book: {:?}", book);
    }

}
