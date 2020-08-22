use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Callback,
};
use yew_router::components::{RouterAnchor};
use yew::services::ConsoleService;

use crate::app::{AppFilter};
use crate::routes::{AppRoute};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct FiltersProps {
    #[prop_or(AppFilter::All)]
    active: AppFilter
}

pub struct Filters {
    link: ComponentLink<Self>,
    props: FiltersProps,
}

pub enum Msg {
    RouteChange
}


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
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let active = self.props.active;
        html! {
            <ul class="filters">
                <li><RouterAnchor<AppRoute> route=AppRoute::Index>{ "All" }</RouterAnchor<AppRoute>></li>
                <li><RouterAnchor<AppRoute> route=AppRoute::Active>{ "Active" }</RouterAnchor<AppRoute>></li>
                <li><RouterAnchor<AppRoute> route=AppRoute::Complete>{ "Complete" }</RouterAnchor<AppRoute>></li>
            </ul>
        }
    }
}
