use naia_bevy_server::transport::webrtc;
use std::net::ToSocketAddrs;
use std::time::Duration;
use bevy_ecs::prelude::*;
use naia_bevy_server::{AuthEvent, ServerConfig};
use tokio::time::sleep;
use tracing::{debug, info, instrument};



pub static SIGNALING_PORT: u16 = 14191;
pub static UDP_PORT: u16 = 14192;

pub struct App {
    server: Server,
    world: World,
}

impl App {
    #[instrument]
    pub fn new(host: &str) -> Self {
        info!("Starting up...");

        let server_addresses = webrtc::ServerAddrs::new(
            (host, SIGNALING_PORT).to_socket_addrs().expect("invalid socket addr").next().expect("did not resolve to singular socket addr"),
            (host, UDP_PORT).to_socket_addrs().expect("invalid socket addr").next().expect("did not resolve to singular socket addr"),
            &*format!("http://{host}:{UDP_PORT}")
        );

        let protocol = protocol();

        let socket = webrtc::Socket::new(
            &server_addresses,
            &protocol.socket
        );

        let mut server = Server::new(
            ServerConfig::default(),
            protocol
        );

        server.listen(socket);
        info!("server has started listening...");


        App {
            server,
            world: World::default(),
        }
    }

    #[instrument(skip(self))]
    pub async fn update(&mut self) {
        let mut events = self.server.receive(self.world.proxy_mut());
        if events.is_empty() {
            sleep(Duration::from_millis(1000 / 128)).await;
            return;
        }

        for (user_key, auth) in events.read::<AuthEvent<Auth>>() {

        }
    }
}

