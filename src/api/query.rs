use crate::api::schema::Book;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn books() -> Vec<Book> {
        vec![
            Book {
                id: "fsdfsd".to_string(),
                title: "fsdfsdf".to_string(),
                author: "fsdfsdfs".to_string(),
            },
            Book {
                id: "fsdfffsd".to_string(),
                title: "fsdfsdf".to_string(),
                author: "fsdfsdfs".to_string(),
            },
        ]
    }
}
