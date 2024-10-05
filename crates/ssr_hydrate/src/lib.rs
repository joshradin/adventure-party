use pages::Index;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::Renderer;

pub use pages::Page;

/// Hydrate with the given page
#[wasm_bindgen]
pub fn hydrate(page: Page) {
    match page {
        Page::Index => {
            let render = Renderer::<Index>::new();
            render.hydrate();
        }
    }
}
