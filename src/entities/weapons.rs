use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    pub fire_rate: f32,
    pub ammo: i32,
    pub last_shot: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,
}

pub fn spawn_weapon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.2, 1.0))),
            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform: Transform::from_translation(position),
            ..default()
        },
        Weapon {
            damage: 10.0,
            fire_rate: 0.5,
            ammo: 30,
            last_shot: 0.0,
        },
    ));
} 