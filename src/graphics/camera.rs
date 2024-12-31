use bevy::prelude::*;

#[derive(Component)]
pub struct FpsCamera {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for FpsCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.5,
            speed: 5.0,
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FpsCamera::default(),
    ));
}

pub fn update_camera(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FpsCamera)>,
) {
    for (mut transform, camera) in query.iter_mut() {
        // Kamera güncelleme mantığı buraya gelecek
    }
} 