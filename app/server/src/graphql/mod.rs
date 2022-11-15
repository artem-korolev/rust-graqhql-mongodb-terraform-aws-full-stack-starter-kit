mod mappers;
mod types;
mod services;
use juniper::RootNode;

use services::mutation;
use services::query;
use services::subscription;

use self::{mutation::MutationRoot, query::QueryRoot, subscription::Subscription};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, Subscription)
}
