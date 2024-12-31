use bevy::prelude::*;

mod core;
mod entities;
mod graphics;
mod physics;
mod networking;
mod ui;
mod utils;

use entities::EntitiesPlugin;
use physics::PhysicsPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(EntitiesPlugin)
        .add_plugins(UIPlugin)
        .run();
} 