use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
};
use yew::services::ConsoleService;
use yewi::components::transition::CSSTransition;
use serde::{Serialize, Deserialize};
use crate::api::session;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct ItemData {
    pub id: u32,
    pub name: String,
    pub complete: bool,
}

#[derive(Properties, Clone, Debug)]
pub struct TodoProps {
    pub id: u32,
}

pub struct Todo {
    link: ComponentLink<Self>,
    props: TodoProps,
    item: ItemData,
}

impl Component for Todo {
    type Message = ();
    type Properties = TodoProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: this is a temp solution for now
        let storage = session::get_session(&"items").unwrap();
        let items: Vec<ItemData> = serde_json::from_value(storage).unwrap_or(Vec::new());
        let item = items.into_iter().find(|item| item.id == props.id).unwrap_or(ItemData {
            id: props.id, name: format!("Item Not Found fod id: {}", props.id), complete: false
        });

        Self {
            link,
            props,
            item: item.clone(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.id != props.id {
            self.props.id = props.id;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let id = self.props.id;
        let name = self.item.name.clone();

        html! {
            <CSSTransition
                name="todo"
                duration=Duration::from_millis(4000)
                appear=Duration::from_millis(2000)
            >
                <div class="todo-view">
                    <p class="todo" id=id>
                        { name }
                    </p>
                </div>
            </CSSTransition>
        }
    }
}

