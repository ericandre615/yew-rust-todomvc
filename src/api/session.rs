use web_sys::{Storage};
use wasm_bindgen::{JsValue};
use serde_json::{Value};

use yew::services::ConsoleService;

pub fn get_session(key: &str) -> Result<Value, String> {
    let window = web_sys::window().expect("Window error");
    let storage = window.session_storage().unwrap().expect("Storage Error");
    let data_str = storage.get_item(key).unwrap().unwrap_or("[]".to_string());
    let data = serde_json::from_str(&data_str).unwrap_or(Value::Array(Vec::new()));//.expect("Error parsing JSON");

    Ok(data)
}

pub fn set_session(key: &str, value: &str) -> Result<(), String> {
    let window = web_sys::window().expect("Window Error");
    let storage = window.session_storage().unwrap().expect("Storage Error");
    storage.set_item(key, value).expect("Error setting Storage");

    Ok(())
}

pub fn clear_session() -> Result<(), String> {
    let window = web_sys::window().expect("Window Error");
    let storage = window.session_storage().unwrap().expect("Storage Error");

    storage.clear();

    Ok(())
}
