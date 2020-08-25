use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
};
use yew::services::ConsoleService;
use crate::components::transition::CSSTransition;
use std::time::Duration;

#[derive(Debug)]
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
}

impl Component for Todo {
    type Message = ();
    type Properties = TodoProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
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
        html! {
            <CSSTransition
                name="todo"
                duration=Duration::from_millis(4000)
                appear=Duration::from_millis(2000)
                enter=Duration::from_millis(3000)
            >
                <p class="todo">
                    { "Render TODO id" } { id }
                </p>
            </CSSTransition>
        }
    }
}

