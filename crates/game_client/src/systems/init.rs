use crate::app::Main;
use crate::resources::Global;
use bevy::color::LinearRgba;
use bevy::prelude::{Assets, Camera2dBundle, Circle, Color, ColorMaterial, Commands, Mesh, ResMut};
use game_shared::messages::Auth;
use log::info;
use naia_bevy_client::transport::webrtc;
use naia_bevy_client::Client;

pub fn init(
    mut commands: Commands,
    mut client: Client<Main>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("Initializing game");
    client.auth(Auth::new("josh", "radin"));
    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());
    client.connect(socket);
    info!("connected to server!");

    commands.spawn(Camera2dBundle::default());
    let mut global = Global::default(); // Load colors
    global.red = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::RED)));
    global.blue = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::BLUE)));
    global.yellow = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::rgb(
        1.0, 1.0, 0.0,
    ))));
    global.green = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::GREEN)));
    global.white = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::WHITE)));
    global.purple = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::rgb(
        1.0, 0.0, 1.0,
    ))));
    global.orange = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::rgb(
        1.0, 0.5, 0.0,
    ))));
    global.aqua = materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba::rgb(
        0.0, 1.0, 1.0,
    ))));

    // Load shapes
    global.circle = meshes.add(Circle::new(6.));

    commands.insert_resource(global);
}
