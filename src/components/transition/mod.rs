use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    Properties,
    Children,
    NodeRef,
};
use web_sys::Element;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use yew::services::{ConsoleService, TimeoutService};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct TransitionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(String::new())]
    pub class: String,
    pub name: String,
    #[prop_or(0)]
    pub duration: u32
}

pub struct Transition {
    link: ComponentLink<Self>,
    props: TransitionProps,
    node_ref: NodeRef,
}

pub enum Msg {
    TestUpdate,
}

impl Component for Transition {
    type Message = Msg;
    type Properties = TransitionProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("Transition Create");
        Self {
            link,
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::info("Transition Update");
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ConsoleService::info("Transition Change");
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        ConsoleService::info("TRansition Mounted/Rendered");
        let element = self.node_ref.cast::<Element>().unwrap();
        let has_attributes = element.has_attributes();
        ConsoleService::info(&format!("NODEREF {:?}", self.node_ref));
        ConsoleService::info(&format!("HASATTRB {:?}", has_attributes));

        let window = web_sys::window().expect("no global `window` exists");

        if first_render {
            ConsoleService::info("Transition First Render");
            // essentially this is Mounted
            self.appear();

            // TODO: figure out best way to have asyn timeout
            // timeout for duration then self.appeared()
           // let a = Closure::wrap(Box::new(move || update_time(&current_time)) as Box<dyn Fn()>);
           //     window
           //             .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)?;
            //let set_appeared() = Closure::wrap(Box::new(move || self.appeared()) as Box<dyn Fn()>);

            //let timeout_id = window.set_timeout_with_callback(|| {
            //    ConsoleService::info("Timeout happened");
            //});
            // TODO: try Closure::once // without the ref_unchcked and foret?
            // UGH:: Lifetime issue :(
            //let set_appeared = Closure::wrap(Box::new(|| {
            //    self.appeared();
            //}) as Box<dyn FnMut()>);

            //window.set_timeout_with_callback_and_timeout_and_arguments_0(
            //    set_appeared.as_ref().unchecked_ref(),//.to_owned(),
            //    2000, // test time
            //).unwrap();
            let set_appeared = Closure::once(move || {
                self.appeared();
            });

            window.set_timeout_with_callback_and_timeout_and_arguments_0(
                set_appeared.as_ref().unchecked_ref(),
                2000, // test time
            ).unwrap();

            // The instance of `Closure` that we created will invalidate its
            // corresponding JS callback whenever it is dropped, so if we were to
            // normally return from `setup_clock` then our registered closure will
            // raise an exception when invoked.
            //
            // Normally we'd store the handle to later get dropped at an appropriate
            // time but for now we want it to be a global handler so we use the
            // `forget` method to drop it without invalidating the closure. Note that
            // this is leaking memory in Rust, so this should be done judiciously!
            //set_appeared().forget();
        }

        self.enter();
        // timeout for duration then self.entered();
    }

    fn destroy(&mut self) {
        ConsoleService::info("Transition Destroy");
        self.exit();
        // timeout for duration then self.exited();
        // not sure how this will work
        // if yew will call destroy before it's removed from dom?
        // after? TODO: look into this
    }

    fn view(&self) -> Html {
        let classes = self.props.class.clone();
        let children = self.props.children.clone();
        let transition_class = format!("transition-{}", self.props.name.to_string());

        ConsoleService::info("Transition Render");

        html! {
            <div
                ref=self.node_ref.clone()
                class=format!("transition {} {}", transition_class, classes)
            >
                { children }
            </div>
        }
    }
}

impl Transition {
    fn apply_transition(&self, classname: &str) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-{}", self.props.name, classname));
    }

    fn remove_transition(&self, classname: &str) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-{}", self.props.name, classname));
    }

    fn appear(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-appear", self.props.name));
    }

    fn appeared(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-appear", self.props.name));
    }

    fn exit(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-exit", self.props.name));
    }

    fn exited(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-exit", self.props.name));
    }

    fn enter(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().add_1(&format!("{}-enter", self.props.name));
    }

    fn entered(&self) {
        let element = self.node_ref.cast::<Element>().unwrap();
        element.class_list().remove_1(&format!("{}-enter", self.props.name));
    }
}
