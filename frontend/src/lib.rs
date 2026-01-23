//! Systematics Frontend
//!
//! Yew-based WASM frontend for visualizing Systematics graphs.

mod api;
mod app;
mod components;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    // Use API-driven app with GraphQL integration
    yew::Renderer::<app::ApiApp>::new().render();
}
