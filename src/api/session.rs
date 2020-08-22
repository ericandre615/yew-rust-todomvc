use web_sys::window;
use web_sys::{Storage};
use serde_json::{Value};

pub get_session(key: &str) -> Result<(), JsValue> {
    let window = window()?;
    let storage = window.session_storage()?;

    let data_str = storage.get_item(key);
    let data: Value = serde_json::from_str(data_str)?;

    Ok(data)
}

pub set_session(key: &str, value: &str) -> Result<(), JsValue> {
    let window = window()?;
    let storage = window.session_storage()?;
    session_storage.set_item(key, value)?;

    Ok(())
}

pub clear_session() -> Result<(), JsValue> {
    let window = window()?;
    let storage = window.session_storage()?;

    storage.clear();

    Ok(())
}
