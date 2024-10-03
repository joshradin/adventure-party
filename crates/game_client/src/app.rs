use bevy::prelude::*;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};
use protocol::protocol;

pub struct Main;

#[derive(SystemSet, Debug, Hash, Clone, Eq, PartialEq)]
struct MainLoop;
#[derive(SystemSet, Debug, Hash, Clone, Eq, PartialEq)]
struct Tick;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // provide the ID selector string here
                canvas: Some("#mygame-canvas".into()),
                // ... any other window properties ...
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            ClientPlugin::<Main>::new(
                ClientConfig::default(),
                protocol(),
            )
        )
        .run();
}