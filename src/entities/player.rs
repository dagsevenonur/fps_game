use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::prelude::*;
use crate::ui::{GameState, GameSettings};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            jump_force: 5.0,
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Oyuncu karakteri
    let player = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Player::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.5, 0.5), // Boy: 1.0, Yarıçap: 0.5
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
    )).id();

    // FPS kamerası
    let camera = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..default()
        },
        PlayerCamera,
    )).id();

    commands.entity(player).push_children(&[camera]);
}

pub fn player_movement(
    state: Res<State<GameState>>,
    settings: Res<GameSettings>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player, &mut Velocity), Without<PlayerCamera>>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut windows: Query<&mut Window>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let mut window = windows.single_mut();

    // Oyun durumuna göre mouse kontrolü
    match *state.get() {
        GameState::Playing => {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
        GameState::Paused => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
            return; // Oyun duraklatıldıysa hareketi devre dışı bırak
        }
    }

    for (mut transform, player, mut velocity) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let forward = transform.forward();
        let right = transform.right();

        // WASD hareketi
        if keyboard_input.pressed(KeyCode::W) {
            direction += forward;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction -= forward;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction -= right;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += right;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            velocity.linvel.y = player.jump_force;
        }

        // Hareket uygula
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            velocity.linvel.x = direction.x * player.speed;
            velocity.linvel.z = direction.z * player.speed;
        } else {
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
        }

        // Mouse kontrolü
        let mut mouse_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            mouse_delta += event.delta;
        }

        if mouse_delta != Vec2::ZERO {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            yaw -= mouse_delta.x * settings.mouse_sensitivity;
            pitch -= mouse_delta.y * settings.mouse_sensitivity;
            pitch = pitch.clamp(-1.5, 1.5);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }
    }
} 