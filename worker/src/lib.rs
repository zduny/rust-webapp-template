use js_utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main_worker() -> Result<(), JsValue> {
    set_panic_hook();

    Ok(())
}
