use crate::app::Main;
use crate::resources::Global;
use bevy::prelude::{Assets, Camera2dBundle, ColorMaterial, Commands, Mesh, ResMut};
use log::info;
use naia_bevy_client::transport::webrtc;
use naia_bevy_client::Client;
use protocol::messages::Auth;

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
    let mut global = Global::default();

    commands.insert_resource(global);
}
