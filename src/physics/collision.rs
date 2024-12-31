use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec3,
    pub is_trigger: bool,
}

pub fn check_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Collider)>,
) {
    // Çarpışma kontrolü mantığı buraya gelecek
} 