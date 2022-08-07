use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(feature = "logging")]
pub fn set_logger() {
    console_log::init_with_level(log::Level::Debug).expect("failed to init logger");
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    #[cfg(feature = "panic_hook")]
    set_panic_hook();

    #[cfg(feature = "logging")]
    set_logger();
    #[cfg(feature = "logging")]
    log::debug!("Hello World!");

    Ok(())
}
