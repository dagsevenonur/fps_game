use bevy::prelude::*;

// Oyun dünyası bileşenleri
#[derive(Component)]
pub struct Player {
    pub health: f32,
    pub ammo: i32,
}

#[derive(Component)]
pub struct Weapon {
    pub damage: f32,
    pub fire_rate: f32,
    pub last_shot: f32,
}

// Oyun sistemleri
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Oyuncu modelini oluştur
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Player {
            health: 100.0,
            ammo: 30,
        },
    ));
}

pub fn update_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        // Oyuncu güncelleme mantığı
    }
} 