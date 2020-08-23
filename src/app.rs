use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};
use yew::services::ConsoleService;

use crate::routes::router;
use crate::components::Filters;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AppFilter {
    All,
    Active,
    Complete,
}

pub struct App {
    link: ComponentLink<Self>,
    item_count: u32,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            // TODO: need to get the item_count from the list component. Could get it from session_storage, but don't think that's a good solution
            // being able to get it from the app component or some other state store in app outside of browser api is better
            item_count: 0,         }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="app" class="todomvc-wrapper">
                <section class="todoapp">
                    { router() }
                <footer class="footer">
                    <span class="todo-count">
                    <strong>{ self.item_count }</strong>
                    { " item(s) left" }
                    </span>
                    <Filters />
                </footer>
                </section>
                <footer class="info" />
            </div>
        }
    }
}

