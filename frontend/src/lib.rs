//! Systematics Frontend
//!
//! Yew-based WASM frontend for visualizing Systematics graphs.

mod app;
mod components;
mod api;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    // Use API-driven app with GraphQL integration
    yew::Renderer::<app::ApiApp>::new().render();
}
