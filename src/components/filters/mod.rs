use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Callback,
};
use yew_router::{
    components::RouterAnchor,
    service::RouteService,
    agent::RouteAgentBridge,
    route::Route
};
use yew::services::ConsoleService;

use crate::app::{AppFilter};
use crate::routes::{AppRoute};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct FiltersProps {
}

pub struct Filters {
    link: ComponentLink<Self>,
    props: FiltersProps,
    current_route: Route,
    _route_agent: RouteAgentBridge,
}

pub enum Msg {
    RouteChange(Route<()>)
}

type RouteLink = RouterAnchor<AppRoute>;

impl Component for Filters {
    type Message = Msg;
    type Properties = FiltersProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let update_route = link.callback(Msg::RouteChange);
        let route_agent = RouteAgentBridge::new(update_route);
        let current_route = RouteService::new().get_route();

        Self {
            link,
            props,
            current_route,
            _route_agent: route_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteChange(route) => {
                self.current_route = route;
            },
        }
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
        let current_route = self.current_route.clone();

        html! {
            <ul class="filters">
                <li><RouteLink route=AppRoute::Index classes=self.active_classname("/")>{ "All" }</RouteLink></li>
                <li><RouteLink route=AppRoute::Active classes=self.active_classname("/active")>{ "Active" }</RouteLink></li>
                <li><RouteLink route=AppRoute::Complete classes=self.active_classname("/complete")>{ "Complete" }</RouteLink></li>
            </ul>
        }
    }
}

impl Filters {
    fn active_classname(&self, route: &str) -> &str {
        match &self.current_route.route {
            curr if curr == route => "selected",
            _ => "not-selected",
        }
    }
}
