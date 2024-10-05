//! Common components

use askama::Template;

/// The game port, a composable component
#[derive(Debug, Template)]
#[template(path = "components/game_port.html")]
pub struct GamePort {
    pub canvas_id: &'static str,
}

#[cfg(test)]
mod tests {
    use crate::components::GamePort;
    use askama::Template;

    #[test]
    fn test_render_game_port() {
        let port = GamePort {
            canvas_id: "mygame-canvas",
        };
        let rendered = port.render().unwrap();
        println!("{}", rendered);
    }
}
