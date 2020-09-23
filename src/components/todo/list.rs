use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
};
use yew::services::ConsoleService;

use yewi::components::transition::CSSTransition;
use crate::components::todo::ListItem;

use std::time::Duration;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children, //TODO: check nested_html!? for restricting the type of a Child//ChildrenRenderer<ListItem>,//Children,
    #[prop_or(String::new())]
    pub class: String,
}

pub struct List {
    link: ComponentLink<Self>,
    props: ListProps,
}

pub enum Msg {}

impl Component for List {
    type Message = Msg;
    type Properties = ListProps;

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
        if self.props != props {
            // TODO: try yewtils neq_assign
            self.props = props; // this is key to updating props that are passed into a component
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();
        let children = self.props.children.clone();

        html! {
            <ul class=format!("list {}", classes)>
                <CSSTransition
                    name="todo-list"
                    enter=Duration::from_millis(600)
                >
                    { children }
                </CSSTransition>
            </ul>
        }
    }
}
