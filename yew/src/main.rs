use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app;
mod components;
mod models;
mod pages;

use app::App;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}
