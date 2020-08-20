use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};
use yew::services::ConsoleService;

use crate::components::form::Input;
use crate::components::todo::{List, ListItem};

#[derive(Debug)]
struct ItemData {
    pub id: u32,
    pub name: String
}

pub struct App {
    link: ComponentLink<Self>,
    items: Vec<ItemData>,
    current_todo: String,
    internal_id: u32,
}

#[derive(Debug)]
pub enum AppMsg {
    Keydown(u32),
    InputChange(String),
    RemoveItem(u32),
}

pub enum Keycode {
    Enter = 13
}

fn is_keycode(value: u32, code: Keycode) -> bool { value == code as u32 }

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            items: vec![
                ItemData { id: 0, name: "Test Todo".to_string() },
                ItemData { id: 99, name: "Second Test Todo".to_string() },
            ],
            current_todo: String::new(),
            internal_id: 1,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // TODO: fix weirdness with update changes seeming "batched" instead of coming in
        // on every change (keystroke) and then being empty when enter happens, even though it should have
        // had several characters by then.. basically you have ot hit ENter twice to get it to add it self.items
        match msg {
            AppMsg::InputChange(input) => {
                ConsoleService::info(&format!("Child Input Changed: {:?}", input));
                self.current_todo = input;
            },
            AppMsg::RemoveItem(item_id) => {
                ConsoleService::info(&format!("REmpove item {:?}", item_id));
                self.items.retain(|item| item.id != item_id);
            },
            AppMsg::Keydown(keycode) => {
                ConsoleService::info(&format!("Key Press {:?}", keycode));
                match keycode {
                    _ if is_keycode(keycode, Keycode::Enter) => {
                        ConsoleService::info("Enter Key was pressed");
                        // okay, worked! now we need to take the input value
                        // and add it to items list
                        let name = self.current_todo.clone();
                        self.current_todo = String::new();
                        ConsoleService::info(&format!("ITEMVAL {:?}", name));
                        ConsoleService::info(&format!("ITEMSNBeORE {:?}", self.items));
                        if !name.is_empty() {
                            self.items.push(ItemData {
                                id: self.internal_id,
                                name,
                            });

                            self.internal_id += 1;
                        }

                        ConsoleService::info(&format!("ITEMAFTER {:?}", self.items));
                    },
                    _ => {}
                }
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // only if props change, no props so example always false
        println!("Change was called with Props {:?}", props);
        false
    }

    fn view(&self) -> Html {
        //let items = vec![ItemData { id: 1, name: "oneP".to_string() }, ItemData { id: 2, name: "twoP".to_string() }];
       // let handle_change_input = self.link.callback(|m: String| -> () {
       //     AppMsg::InputChange(m);
       // });
       // Callback::from(|v| {
       //     ConsoleService::info(&format!("Parent got InputChange {:?}", v));
       // });
       // let handle_change_input = Callback::from(&self.handle_change_input);
        ConsoleService::info("APP RENDERED");

        html! {
            <div
                id="app"
                class="todomvc-wrapper"
                onkeydown=self.link.callback(|e: KeyboardEvent| AppMsg::Keydown(e.key_code()))
            >
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        <Input class="new-todo"
                            initial_value=self.current_todo.clone()
                            placeholder="What needs to be done?"
                            handle_change=self.link.callback(AppMsg::InputChange)
                        />
                    </header>
                    <List class="todo-list">
                        {
                            for self.items.iter().map(|litem| {
                                ConsoleService::info(&format!("Iter OVER ITEMS {:?}", litem));
                                let ItemData { name, id } = litem;
                                html! {
                                    <ListItem
                                        id=id class="todo"
                                        item=name
                                        handle_remove=self.link.callback(AppMsg::RemoveItem)
                                    />
                                }
                            }).collect::<Vec<Html>>()
                        }
                    </List>
                    <footer class="footer">
                        <span class="todo-count">
                            /* TODO: only count Active */
                            <strong>{ self.items.len() }</strong>
                            { " item(s) left" }
                        </span>
                    </footer>
                </section>
                <footer class="info" />
                <ul>
                    {
                        for self.items.iter().map(|litem| {
                            ConsoleService::info(&format!("Iter OVER ITEMS {:?}", litem));
                            let ItemData { name, id } = litem;
                            html! {
                                <li id=id class="todo">{ name }</li>
                            }
                        }).collect::<Vec<Html>>()
                    }
                </ul>
            </div>
        }
    }
}

