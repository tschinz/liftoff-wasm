use wasm_bindgen::prelude::*;

pub mod app;
pub mod components;
pub mod model;
pub mod timer;
pub mod update;
pub mod view;

#[wasm_bindgen(start)]
pub fn run() {
  yew::Renderer::<app::App>::new().render();
}
