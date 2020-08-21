use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Callback,
};
use yew::services::ConsoleService;
use crate::components::Button;
use crate::components::form::Checkbox;

#[derive(Properties, Clone, Debug)]
pub struct ListItemProps {
    #[prop_or(0)]
    pub id: u32,
    #[prop_or(String::new())]
    pub item: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_remove: Callback<u32>,
    #[prop_or(Callback::noop())]
    pub handle_complete: Callback<(u32, bool)>,
}

pub struct ListItem {
    link: ComponentLink<Self>,
    props: ListItemProps,
    completed: bool,
}

pub enum Msg {
    ToggleComplete(bool),
    Clicked(bool),
}


impl Component for ListItem {
    type Message = Msg;
    type Properties = ListItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            completed: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleComplete(complete) => {
                self.completed = complete;
                self.props.handle_complete.emit((self.props.id, self.completed));
            },
            Msg::Clicked(clicked) => {
                ConsoleService::info(&format!("Schedule remove for {:?}", self.props.id));
                self.props.handle_remove.emit(self.props.id);
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let id = self.props.id;
        let item = self.props.item.clone();
        let classes = self.props.class.clone();
        let completed = if self.completed { "completed" } else { "" };

        html! {
            <li class=format!("list-item {} {}", classes, completed)>
                <div class="view" id=id>
                    <Checkbox
                        class="toggle"
                        value=item.clone()
                        handle_change=self.link.callback(Msg::ToggleComplete)
                    />

                    <label><span class="list-name">{ item }</span></label>
                    <Button class="destroy" handle_click=self.link.callback(Msg::Clicked) />
                </div>
            </li>
        }
    }
}
