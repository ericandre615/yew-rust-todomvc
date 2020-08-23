#![recursion_limit = "512"]

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use yew;

mod app;
mod components;
mod views;
mod routes;
mod api;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
