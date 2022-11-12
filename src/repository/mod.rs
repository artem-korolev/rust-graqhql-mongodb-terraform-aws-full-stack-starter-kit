pub mod schema;
use mongodb::{options::ClientOptions, Client};
use mongodb::{Collection, Database};

// // Get a handle to a collection of `Book`.
// pub async fn get_books_collection() -> Result<Collection<schema::Book>, mongodb::error::Error> {
//     let client_options = ClientOptions::parse("mongodb://localhost:27018").await?;
//     // Get a handle to the deployment.
//     let client = Client::with_options(client_options)?;
//     let db = client.database("mydb");
//     Ok(db.collection::<schema::Book>("books"))
// }

#[derive(Debug, Clone)]
pub struct Repository {
    db: Database,
}

impl juniper::Context for Repository {}

impl Repository {
    pub async fn new() -> Result<Repository, mongodb::error::Error> {
        let client_options = ClientOptions::parse("mongodb://localhost:27018").await?;
        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        let db = client.database("mydb");
        Ok(Repository { db: db })
    }

    pub fn get_books_collection(&self) -> Collection<schema::Book> {
        self.db.collection::<schema::Book>("books")
    }
}
