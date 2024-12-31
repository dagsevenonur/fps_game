use bevy::prelude::*;
use tokio::net::TcpListener;

#[derive(Resource)]
pub struct NetworkServer {
    pub running: bool,
    pub port: u16,
}

impl Default for NetworkServer {
    fn default() -> Self {
        Self {
            running: false,
            port: 7777,
        }
    }
}

pub fn setup_server(mut commands: Commands) {
    commands.insert_resource(NetworkServer::default());
}

pub fn update_server() {
    // Sunucu güncelleme mantığı buraya gelecek
} 