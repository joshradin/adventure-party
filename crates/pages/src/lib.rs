//! Contains main yew pages

use components::game::Game;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html, BaseComponent, Html};

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub enum Page {
    Index,
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Page.{self:?}")
    }
}

pub trait PageComponent: BaseComponent {
    fn page() -> Page;
}

#[function_component]
pub fn Index() -> Html {
    html! {
        <Game />
    }
}

impl PageComponent for Index {
    fn page() -> Page {
        Page::Index
    }
}
