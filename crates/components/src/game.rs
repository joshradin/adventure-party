//! Game-related components

use yew::{function_component, html, AttrValue, Html, Properties};

/// Game properties
#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::Static("/game_client/game_client.js"))]
    pub game_path: AttrValue,
}

#[function_component]
pub fn Game(props: &Props) -> Html {
    html! {
        <div>
            <canvas id="mygame-canvas" />
            <script type="module">
                <>
                    {format!("import app from \"{}\"", props.game_path)}
                    {r#"

                        async function start() {
                            await app({})
                        }
                        start()
                    "#}
                </>
            </script>
        </div>
    }
}
