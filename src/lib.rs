#![cfg(target_arch = "wasm32")]

mod app;

use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render(app));
}
