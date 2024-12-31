use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
           .add_systems(Startup, setup_physics);
    }
}

fn setup_physics(mut configuration: ResMut<RapierConfiguration>) {
    configuration.gravity = Vec3::new(0.0, -9.81, 0.0);
} 