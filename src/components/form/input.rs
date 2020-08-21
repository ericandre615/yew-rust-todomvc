use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    ChangeData,
    Callback,
};
use yew::services::ConsoleService;

#[derive(Properties, Clone, Debug)]
pub struct InputProps {
    #[prop_or(String::new())]
    pub initial_value: String,
    #[prop_or(String::new())]
    pub value: String,
    #[prop_or(String::new())]
    pub label: String,
    #[prop_or(String::new())]
    pub placeholder: String,
    #[prop_or(String::new())]
    pub class: String,
    #[prop_or(Callback::noop())]
    pub handle_change: Callback<String>,
}

pub struct Input {
    link: ComponentLink<Self>,
    props: InputProps,
    value: String,
}

pub enum Msg {
    UpdateValue(ChangeData),
}


impl Component for Input {
    type Message = Msg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = if !props.value.is_empty() {
            props.value.clone()
        } else {
            props.initial_value.clone()
        };

        Self {
            link,
            props,
            value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateValue(change) => {
                match change {
                    ChangeData::Value(value) => {
                        self.value = value.clone();
                        self.props.handle_change.emit(self.value.clone());
                    },
                    _ => {}
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO: this works, I guess, but not happy with it
        if self.props.value != props.value {
            self.props.value = props.value.clone();
            self.value = props.value;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let label = self.props.label.clone();
        let placeholder = self.props.placeholder.clone();
        let classes = self.props.class.clone();

        html! {
            <div>
                <label>{ label }</label>
                <input
                    type="text"
                    class=classes
                    onchange=self.link.callback(|v: ChangeData| Msg::UpdateValue(v))
                    value={ self.value.clone() }
                    placeholder={ placeholder }
                />
            </div>
        }
    }
}
