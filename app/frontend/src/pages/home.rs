use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

use serde_json::Value;
use yew::prelude::*;

use self::home_page_data::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../model/api/schema.graphql",
    query_path = "./graphql/home.graphql",
    response_derives = "Debug"
)]
struct HomePageData;

pub enum Msg {
    SetState(FetchState<ResponseData>),
    GetData,
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
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

pub struct Home {
    data: FetchState<ResponseData>,
}

fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s))
}

async fn fetch_gql_data() -> Result<ResponseData, FetchError> {
    let client = reqwest::Client::new();
    let variables = home_page_data::Variables {};

    let response =
        post_graphql::<HomePageData, _>(&client, "http://127.0.0.1:3000/graphql", variables)
            .await
            .map_err(|err| {
                log(&format!("Could not fetch puppies. error: {:?}", err));
                Value::Null
            })?;
    Ok(render_response(response))
}

fn render_response(
    response: graphql_client::Response<home_page_data::ResponseData>,
) -> ResponseData {
    response.data.expect("error")
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: FetchState::NotFetching,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::GetData);
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetState(fetch_state) => {
                self.data = fetch_state;

                true
            }
            Msg::GetData => {
                ctx.link().send_future(async {
                    match fetch_gql_data().await {
                        Ok(data) => Msg::SetState(FetchState::Success(data)),
                        Err(err) => Msg::SetState(FetchState::Failed(err)),
                    }
                });

                ctx.link().send_message(Msg::SetState(FetchState::Fetching));

                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.data {
            FetchState::NotFetching => html! { "NotFetching" },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(home_data) => self.view_home(home_data),
            FetchState::Failed(err) => html! { err },
        }
    }
}
impl Home {
    fn view_home(&self, data: &ResponseData) -> Html {
        let books = data.books.iter().map(|book| {
            html! {
                <div class="book">
                    { book.title.to_owned() + " :: " + &book.author }
                </div>
            }
        });
        html! {
            <>
                { for books }
            </>
        }
    }
}
