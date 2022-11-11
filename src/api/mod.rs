mod mutation;
mod query;
use juniper::{EmptySubscription, RootNode};

use self::{mutation::MutationRoot, query::QueryRoot};

pub mod schema;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<()>::new())
}
