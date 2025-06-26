use wasm_bindgen::prelude::*;
use yew::Renderer;

// lib.rs required for cdylib crate-type

pub mod app;
pub mod components;
pub mod data;
pub mod models;
pub mod pages;
pub mod styles;

pub use app::App;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Starting Yew application...");

    Renderer::<App>::new().render();

    log::info!("Yew application rendered");
    Ok(())
}
