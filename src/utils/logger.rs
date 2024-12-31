use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

pub struct CustomLogPlugin;

impl Plugin for CustomLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
           .add_systems(Update, log_fps);
    }
}

fn log_fps(diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            info!("FPS: {value:.2}");
        }
    }
} 