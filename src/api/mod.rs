mod subscription;
mod mutation;
mod query;
use juniper::{RootNode};

use self::{mutation::MutationRoot, query::QueryRoot, subscription::Subscription};

pub mod schema;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, Subscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, Subscription)
}
