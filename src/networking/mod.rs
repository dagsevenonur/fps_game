use bevy::prelude::*;
use tokio::net::{TcpListener, TcpStream};

mod client;
mod server;
mod protocol;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_networking);
    }
}

fn update_networking() {
    // Network güncelleme mantığı buraya gelecek
} 