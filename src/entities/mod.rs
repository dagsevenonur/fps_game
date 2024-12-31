use bevy::prelude::*;
use crate::ui::GameState;

mod world;
mod player;

pub use world::*;
pub use player::*;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_world, spawn_player))
           .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
} 