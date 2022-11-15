#[derive(juniper::GraphQLInputObject)]
pub struct NewBook {
    pub title: String,
    pub author: String,
}
