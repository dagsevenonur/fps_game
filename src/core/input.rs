use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

// Input bileşenleri
#[derive(Resource, Default)]
pub struct InputState {
    pub mouse_delta: Vec2,
    pub movement_input: Vec3,
}

// Input sistemleri
pub fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<InputState>,
) {
    let mut movement = Vec3::ZERO;

    // WASD hareketi
    if keyboard_input.pressed(KeyCode::W) {
        movement.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement.x += 1.0;
    }

    // Zıplama
    if keyboard_input.pressed(KeyCode::Space) {
        movement.y += 1.0;
    }

    input_state.movement_input = movement.normalize_or_zero();
}

pub fn handle_mouse_input(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut input_state: ResMut<InputState>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }
    input_state.mouse_delta = delta;
}

// Mouse görünürlüğünü kontrol et
pub fn toggle_cursor(
    mut windows: Query<&mut Window>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Tab) {
        let mut window = windows.single_mut();
        window.cursor.visible = !window.cursor.visible;
        window.cursor.grab_mode = if window.cursor.visible {
            bevy::window::CursorGrabMode::None
        } else {
            bevy::window::CursorGrabMode::Locked
        };
    }
} 