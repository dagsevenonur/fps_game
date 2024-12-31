use bevy::prelude::*;

mod renderer;
mod camera;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_graphics);
    }
}

fn update_graphics() {
    // Grafik güncelleme mantığı buraya gelecek
} 