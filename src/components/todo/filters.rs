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

//use crate::components::todo::Filter;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct FiltersProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
}

pub struct Filters {
    link: ComponentLink<Self>,
    props: FiltersProps,
}

pub enum Msg {}

impl Component for Filters {
    type Message = Msg;
    type Properties = FiltersProps;

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
                { children }
            </ul>
        }
    }
}
