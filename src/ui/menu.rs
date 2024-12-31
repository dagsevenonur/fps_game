use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::CursorGrabMode;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

#[derive(Resource)]
pub struct GameSettings {
    pub mouse_sensitivity: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.002,
        }
    }
}

pub fn pause_menu_system(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    mut windows: Query<&mut Window>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        
        match state.get() {
            GameState::Playing => {
                // Oyunu duraklat
                next_state.set(GameState::Paused);
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            GameState::Paused => {
                // Oyuna devam et
                next_state.set(GameState::Playing);
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}

pub fn draw_pause_menu(
    mut contexts: Query<&mut bevy_egui::EguiContext>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings: ResMut<GameSettings>,
    mut exit: EventWriter<AppExit>,
) {
    if *state.get() == GameState::Paused {
        let mut context = contexts.single_mut();
        
        egui::Window::new("Pause Menu")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(context.get_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Oyun Duraklatıldı");
                    ui.add_space(20.0);
                    
                    ui.label("Mouse Hassasiyeti:");
                    ui.add(egui::Slider::new(&mut settings.mouse_sensitivity, 0.0001..=0.01));
                    
                    ui.add_space(20.0);
                    if ui.button("Devam Et").clicked() {
                        next_state.set(GameState::Playing);
                    }
                    
                    if ui.button("Oyundan Çık").clicked() {
                        exit.send(AppExit);
                    }
                });
            });
    }
} 