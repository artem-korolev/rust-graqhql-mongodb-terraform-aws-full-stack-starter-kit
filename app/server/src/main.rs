mod api;
mod repository;
mod server;

extern crate log;
extern crate pretty_env_logger;

use crate::{server::create_server, repository::Repository};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();

    let schema = api::create_schema();
    let db = Repository::new().await.unwrap();
    let server = create_server(schema, db, addr).await;

    println!("Listening on http://{addr}");

    if let Err(e) = server.await {
        eprintln!("server error: {e}")
    }
}

// async fn test_database() {
//     let book = repository::schema::Book {
//         title: "The Grapes of Wrath".to_string(),
//         author: "John Steinbeck".to_string(),
//         id: None,
//     };
//     let repo = get_books_collection().await.unwrap();
//     repo.insert_one(book, None).await.unwrap();

//     let mut cursor = repo.find(None, None).await.unwrap();

//     // Iterate over the results of the cursor.
//     while let Some(book) = cursor.try_next().await.unwrap() {
//         println!("book: {:?}", book);
//     }

// }
