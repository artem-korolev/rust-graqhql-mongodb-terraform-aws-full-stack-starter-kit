use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use serde_json::Value;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub enum Msg<T> {
    SetState(FetchState<T>),
    GetData,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: Value,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<Value> for FetchError {
    fn from(value: Value) -> Self {
        Self { err: value }
    }
}

pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s))
}

pub trait GQLQuery<T> {}

impl<TGraphQLQuery> dyn GQLQuery<TGraphQLQuery>
where
    TGraphQLQuery: GraphQLQuery,
{
    pub async fn fetch_gql_data(
        variables: <TGraphQLQuery as GraphQLQuery>::Variables,
    ) -> Result<<TGraphQLQuery as GraphQLQuery>::ResponseData, FetchError> {
        let client = reqwest::Client::new();
        // let variables = home_page_data::Variables {};

        let response =
            post_graphql::<TGraphQLQuery, _>(&client, "http://127.0.0.1:3000/graphql", variables)
                .await
                .map_err(|err| {
                    log(&format!("Could not fetch puppies. error: {:?}", err));
                    Value::Null
                })?;
        Ok(Self::render_response(response))
    }

    fn render_response(
        response: graphql_client::Response<<TGraphQLQuery as GraphQLQuery>::ResponseData>,
    ) -> <TGraphQLQuery as GraphQLQuery>::ResponseData {
        response.data.expect("error")
    }
}
