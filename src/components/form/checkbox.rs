use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    ChangeData,
    MouseEvent,
    Callback,
};
use yew::services::ConsoleService;

#[derive(Properties, Clone)]
pub struct CheckboxProps {
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(false)]
    pub checked: bool,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<bool>,
}

pub struct Checkbox {
    link: ComponentLink<Self>,
    props: CheckboxProps,
    checked: bool,
}

pub enum Msg {
    ToggleChecked,
}


impl Component for Checkbox {
    type Message = Msg;
    type Properties = CheckboxProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let checked = props.checked;
        Self {
            link,
            props,
            checked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleChecked => {
                self.checked = !self.checked;
                self.props.handle_change.emit(self.checked);
            },
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // only if props change, no props so example always false
        false
    }

    fn view(&self) -> Html {
        let value = self.props.value.clone();
        let classes = self.props.class.clone();

        html! {
            <input
                type="checkbox"
                class=classes
                onclick=self.link.callback(|_| Msg::ToggleChecked)
                checked={ self.checked }
                value=value
            />
        }
    }
}
