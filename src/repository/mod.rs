pub mod schema;
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

// Get a handle to a collection of `Book`.
pub async fn get_books_collection() -> Result<Collection<schema::Book>, mongodb::error::Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27018").await?;
    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    let db = client.database("mydb");
    Ok(db.collection::<schema::Book>("books"))
}
