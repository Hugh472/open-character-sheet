use wasm_bindgen::prelude::*;
use yew::start_app;

mod app;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    start_app::<app::App>();
}