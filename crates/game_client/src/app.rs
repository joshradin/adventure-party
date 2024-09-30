use log::info;
use adventure_party_naia_world::{Entity, World};
use naia_client::transport::webrtc;
use naia_client::ClientConfig;
use naia_macroquad_shared::messages::Auth;
use naia_macroquad_shared::protocol;

type Client = naia_client::Client<Entity>;

pub struct App {
    client: Client,
    world: World,
}

impl App {
    pub fn new() -> Self {
        info!("Starting up...");

        let protocol = protocol();
        let socket = webrtc::Socket::new("http://127.0.0.1:14191", &protocol.socket);
        let mut client = Client::new(
            ClientConfig::default(),
            protocol
        );
        client.auth(Auth::new("joshua", "12345678"));
        client.connect(socket);

        Self {
            client,
            world: Default::default(),
        }
    }
}