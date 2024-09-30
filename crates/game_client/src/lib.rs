use crate::app::App;
use log::{info, Level};
use macroquad::prelude::next_frame;
use macroquad::Window;
use wasm_bindgen::prelude::wasm_bindgen;

mod app;
mod interp;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
    info!("attempting to start game..");
    Window::new("AdventureParty", run())
}

async fn run() {
    let mut app = App::new();
    info!("app started...");
    loop {
        next_frame().await;
    }
}
