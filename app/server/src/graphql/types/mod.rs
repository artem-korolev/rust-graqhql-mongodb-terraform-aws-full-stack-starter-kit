pub mod input;

use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
}
