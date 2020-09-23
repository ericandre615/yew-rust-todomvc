use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
};

#[derive(Properties, Clone)]
pub struct ButtonProps {
    #[prop_or(String::new())]
    pub class: String,
}

pub struct FollowButton {
    props: ButtonProps,
}

pub enum Msg {
}

impl Component for FollowButton {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();

        html! {
            <a
                class=format!("github-button {}", classes)
                href="https://github.com/ericandre615"
                aria-label="Follow @ericandre615 on GitHub"
            >
                    { "Follow @ericandre615" }
            </a>
        }
    }
}
