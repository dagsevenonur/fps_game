use bevy::prelude::*;
use tokio::net::TcpStream;

#[derive(Resource)]
pub struct NetworkClient {
    pub connected: bool,
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self {
            connected: false,
        }
    }
}

pub fn setup_client(mut commands: Commands) {
    commands.insert_resource(NetworkClient::default());
}

pub fn update_client() {
    // İstemci güncelleme mantığı buraya gelecek
} 