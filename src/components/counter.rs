use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender
};

pub struct Counter {
    link: ComponentLink<Self>,
    value: i64,
}

pub enum Msg {
    AddOne,
    SubtractOne,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value +=1,
            Msg::SubtractOne => self.value -= 1,
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // only if props change, no props so example always false
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button class="btn btn-success" onclick=self.link.callback(|_| Msg::AddOne)>{ "+" }</button>
                <button class="btn btn-failure" onclick=self.link.callback(|_| Msg::SubtractOne)>{ "-" }</button>
                <span class="counter counter-value">{ self.value }</span>
            </div>
        }
    }
}
