use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
    NodeRef,
    Callback,
};
use crate::components::transition::Transition;
use web_sys::Element;
use yew::services::{ConsoleService};
use std::time::Duration;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct CSSTransitionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    pub name: String,
    #[prop_or(Duration::from_millis(0))]
    pub duration: Duration,
    #[prop_or_default]
    pub appear: Option<Duration>,
    #[prop_or_default]
    pub enter: Option<Duration>,
    #[prop_or_default]
    pub exit: Option<Duration>,
}

pub struct CSSTransition {
    link: ComponentLink<Self>,
    props: CSSTransitionProps,
    node_ref: NodeRef, // Maybe Transition component passes it's node ref? Maybe doesn't matter
}

pub enum Msg {
    Appear,
    Appearing,
    Appeared,
    Exit,
    Exiting,
    Exited,
    Enter,
    Entering,
    Entered,
}

impl Component for CSSTransition {
    type Message = Msg;
    type Properties = CSSTransitionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link.clone(),
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Appear => {
                self.appear();
            },
            Msg::Appearing => {
                self.appearing();
            },
            Msg::Appeared => {
                self.appeared();
            },
            Msg::Entering => {
                self.entering();
            },
            Msg::Enter => {
                self.enter();
            },
            Msg::Entered => {
                self.entered();
            },
            Msg::Exiting => {
                self.exiting();
            },
            Msg::Exit => {
                self.exit();
            },
            Msg::Exited => {
                self.exited();
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ConsoleService::info("CSSTransition Change");
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();
        let name = self.props.name.clone();
        let duration = self.props.duration.clone();
        let appear = self.props.appear.clone();
        let enter = self.props.enter.clone();
        let exit = self.props.exit.clone();
        let children = self.props.children.clone();
        let transition_class = format!("transition-{}", self.props.name.to_string());

        ConsoleService::info("CSSTransition Render");

        html! {
            <Transition
                ref=self.node_ref.clone()
                class=classes
                name=name
                duration=duration
                appear=appear
                enter=enter
                exit=exit
                on_appearing=self.link.callback(|_| Msg::Appearing)
                on_appear=self.link.callback(|_| Msg::Appear)
                on_appeared=self.link.callback(|_| Msg::Appeared)
                on_entering=self.link.callback(|_| Msg::Entering)
                on_enter=self.link.callback(|_| Msg::Enter)
                on_entered=self.link.callback(|_| Msg::Entered)
                on_exiting=self.link.callback(|_| Msg::Exiting)
                on_exit=self.link.callback(|_| Msg::Exit)
                on_exited=self.link.callback(|_| Msg::Exited)
            >
                { children }
            </Transition>
        }
    }
}

impl CSSTransition {
    fn apply_transition(&self, classname: &str) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-{}", self.props.name, classname));
    }

    fn remove_transition(&self, classname: &str) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-{}", self.props.name, classname));
    }

    fn appearing(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-appear", self.props.name));
        element.class_list().add_1(&format!("{}-appearing", self.props.name));
    }

    fn appear(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-appear", self.props.name));
    }

    fn appeared(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-appearing", self.props.name));
        element.class_list().add_1(&format!("{}-appeared", self.props.name));
    }

    fn exiting(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-exit", self.props.name));
        element.class_list().add_1(&format!("{}-exiting", self.props.name));
    }

    fn exit(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-exit", self.props.name));
    }

    fn exited(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-exiting", self.props.name));
        element.class_list().add_1(&format!("{}-exited", self.props.name));
    }

    fn entering(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-enter", self.props.name));
        element.class_list().add_1(&format!("{}-entering", self.props.name));
    }

    fn enter(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-enter", self.props.name));
    }

    fn entered(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-entering", self.props.name));
        element.class_list().add_1(&format!("{}-entered", self.props.name));
    }
}
