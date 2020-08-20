use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
};
//use yew::html::ChildrenRenderer;
use yew::services::ConsoleService;

use crate::components::todo::ListItem;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ListProps {
    #[prop_or_default]
    pub children: Children,//ChildrenRenderer<ListItem>,//Children,
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
        ConsoleService::info("Change for LIST");
        ConsoleService::info(&format!("Self childs {:?}", self.props));
        ConsoleService::info(&format!("Props Cihlds {:?}", props));
        if self.props.children != props.children {
            ConsoleService::info("CHLDREN DIFFED");
            self.props = props;
            true
        } else {
            ConsoleService::info("CHLD SAME");
            false
        }
    }

    fn view(&self) -> Html {
        ConsoleService::info(&format!("LIST RENDERING CHLIDREN {:?}", self.props.children));
        let classes = self.props.class.clone();
        let children = self.props.children.clone();
        html! {
            <ul class=format!("list {}", classes)>
                { children }
            </ul>
        }
    }
}
