use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
    Properties,
};
use yew::services::ConsoleService;

use crate::components::form::Input;
use crate::components::todo::{List, ListItem};
use crate::app::{AppFilter};

//#[derive(Debug, Clone)]
//pub enum AppFilter {
//    Active,
//    Complete,
//    All,
//}

#[derive(Debug)]
struct ItemData {
    pub id: u32,
    pub name: String,
    pub complete: bool,
}

#[derive(Properties, Clone, Debug)]
pub struct IndexProps {
    pub filter: AppFilter,
}

pub struct Index {
    link: ComponentLink<Self>,
    items: Vec<ItemData>,
    current_todo: String,
    internal_id: u32,
    props: IndexProps,
}

#[derive(Debug)]
pub enum AppMsg {
    Keydown(u32),
    InputChange(String),
    RemoveItem(u32),
    ToggleComplete((u32, bool)),
}

pub enum Keycode {
    Enter = 13
}

fn is_keycode(value: u32, code: Keycode) -> bool { value == code as u32 }

impl Component for Index {
    type Message = AppMsg;
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            items: Vec::new(),
            current_todo: String::new(),
            internal_id: 0,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // TODO: fix weirdness with update changes seeming for Keydown listener (always have to double hit Enter to add item)
        match msg {
            AppMsg::InputChange(input) => {
                self.current_todo = input;
            },
            AppMsg::RemoveItem(item_id) => {
                self.items.retain(|item| item.id != item_id);
            },
            AppMsg::ToggleComplete((item_id, _)) => {
                for item in &mut self.items {
                    if item.id == item_id {
                        item.complete = !item.complete;
                    }
                }
            },
            AppMsg::Keydown(keycode) => {
                match keycode {
                    _ if is_keycode(keycode, Keycode::Enter) => {
                        let name = self.current_todo.clone();
                        self.current_todo = String::new();

                        if !name.is_empty() {
                            self.items.push(ItemData {
                                id: self.internal_id,
                                name,
                                complete: false,
                            });

                            self.internal_id += 1;
                        }
                    },
                    _ => {}
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.filter != props.filter {
            self.props.filter = props.filter;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let items = self.render_items(&self.props.filter);
        html! {
            <>
                <header
                    class="header"
                    onkeydown=self.link.callback(|e: KeyboardEvent| AppMsg::Keydown(e.key_code()))
                >
                    <h1>{ "todos" }</h1>
                    <Input class="new-todo"
                        value=self.current_todo.clone()
                        placeholder="What needs to be done?"
                        handle_change=self.link.callback(AppMsg::InputChange)
                    />
                </header>
                <section class="main">
                    <List class="todo-list">
                        { items }
                    </List>
                </section>
            </>
        }
    }
}

impl Index {
    fn render_items(&self, filter: &AppFilter) -> Vec<Html> {
        self.items.iter()
        .filter(|item| {
            match filter {
                AppFilter::Active => !item.complete,
                AppFilter::Complete => item.complete,
                AppFilter::All => true,
            }
        })
        .map(|litem| {
            let ItemData { name, id, complete } = litem;
            html! {
                <ListItem
                    key={ *id as i128 }
                    id=id
                    class="todo"
                    item=name
                    handle_remove=self.link.callback(AppMsg::RemoveItem)
                    handle_complete=self.link.callback(AppMsg::ToggleComplete)
                />
            }
        }).collect::<Vec<Html>>()
    }
}
