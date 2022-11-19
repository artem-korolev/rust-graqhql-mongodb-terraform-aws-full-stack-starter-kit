use graphql_client::GraphQLQuery;

use crate::core::{FetchState, GQLQuery, Msg};
use yew::prelude::*;

use self::home_page_query::ResponseData;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../model/api/schema.graphql",
    query_path = "./graphql/home.graphql",
    response_derives = "Debug"
)]
struct HomePageQuery;

pub struct Home {
    data: FetchState<ResponseData>,
}

impl Component for Home {
    type Message = Msg<ResponseData>;
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
                    match <dyn GQLQuery<HomePageQuery>>::fetch_gql_data(
                        home_page_query::Variables {},
                    )
                    .await
                    {
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
                    <span>{ "ObjectId(" }{ &book.id }{") = "}</span>
                    <span>{ &book.title }{ " :: " }{ &book.author }</span>
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
