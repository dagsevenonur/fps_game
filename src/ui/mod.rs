use bevy::prelude::*;
use bevy_egui::EguiPlugin;
mod menu;
mod hud;

pub use menu::*;
pub use hud::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
           .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
           .init_resource::<GameSettings>()
           .init_resource::<PlayerStats>()
           .add_state::<GameState>()
           .add_systems(Update, (
               pause_menu_system,
               draw_pause_menu,
               draw_hud.run_if(in_state(GameState::Playing))
           ));
    }
} 