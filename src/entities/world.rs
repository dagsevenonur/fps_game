use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Zemin
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Collider::cuboid(25.0, 0.1, 25.0),
        RigidBody::Fixed,
    ));

    // Ana ışık
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Ortam ışığı
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
} 