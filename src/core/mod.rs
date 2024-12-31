mod game;
mod input;
mod config;

pub use game::*;
pub use input::*;
pub use config::*;

use bevy::prelude::*;

// Core plugin
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameConfig>()
            .add_systems(Update, update_game_core);
    }
}

// Oyun konfigürasyonu
#[derive(Resource, Debug)]
pub struct GameConfig {
    pub mouse_sensitivity: f32,
    pub movement_speed: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.5,
            movement_speed: 5.0,
        }
    }
}

// Ana oyun güncelleme sistemi
fn update_game_core(
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    // Temel oyun mantığı buraya gelecek
} 