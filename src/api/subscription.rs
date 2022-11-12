use std::pin::Pin;

use futures::Stream;
use juniper::{graphql_subscription, FieldError};

use crate::repository::Repository;

pub struct Subscription;

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = Repository)]
impl Subscription {
    async fn hello_world() -> StringStream {
        let stream = futures::stream::iter(vec![
            Ok(String::from("Hello")),
            Ok(String::from("World!"))
        ]);
        Box::pin(stream)
    }
}
