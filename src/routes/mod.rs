use yew_router::{Switch, router::Router};
use yew::prelude::{Html, html};

use crate::app::{AppFilter};
use crate::views::{index::Index, todo::Todo};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/todo/{id}"]
    Todo(u32),
    #[to = "/complete"]
    Complete,
    #[to = "/active"]
    Active,
    #[to = "/"]
    Index,
}

pub fn router() -> Html {
    html! {
        <Router<AppRoute, ()>
            render = Router::render(|route: AppRoute| {
                match route {
                    AppRoute::Todo(id) => html! { <Todo id=id /> },
                    AppRoute::Active => html! { <Index filter=AppFilter::Active />  },
                    AppRoute::Complete => html!{ <Index filter=AppFilter::Complete /> },
                    AppRoute::Index => html!{ <Index filter=AppFilter::All /> },
                }
            })
        />
    }
}
