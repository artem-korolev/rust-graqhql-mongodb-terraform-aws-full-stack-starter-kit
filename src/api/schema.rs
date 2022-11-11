use juniper::GraphQLObject;

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
