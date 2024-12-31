use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    PlayerJoin {
        id: u32,
        position: Vec3,
    },
    PlayerMove {
        id: u32,
        position: Vec3,
        rotation: Quat,
    },
    PlayerLeave {
        id: u32,
    },
    GameState {
        players: Vec<PlayerState>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: u32,
    pub position: Vec3,
    pub rotation: Quat,
    pub health: f32,
} 