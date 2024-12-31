use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Oyun ayarları
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct GameSettings {
    // Grafik ayarları
    pub resolution: (u32, u32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub fov: f32,

    // Ses ayarları
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,

    // Kontrol ayarları
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            fullscreen: false,
            vsync: true,
            fov: 90.0,
            master_volume: 1.0,
            music_volume: 0.8,
            sfx_volume: 1.0,
            mouse_sensitivity: 0.5,
            invert_y: false,
        }
    }
}

// Ayarları kaydet/yükle
pub fn save_settings(settings: &GameSettings) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(settings)?;
    std::fs::write("settings.json", json)?;
    Ok(())
}

pub fn load_settings() -> Result<GameSettings, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("settings.json")?;
    let settings = serde_json::from_str(&json)?;
    Ok(settings)
}

// Ayarları uygula
pub fn apply_settings(
    settings: Res<GameSettings>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    
    window.resolution.set(settings.resolution.0 as f32, settings.resolution.1 as f32);
    window.mode = if settings.fullscreen {
        bevy::window::WindowMode::Fullscreen
    } else {
        bevy::window::WindowMode::Windowed
    };
    
    window.present_mode = if settings.vsync {
        bevy::window::PresentMode::AutoVsync
    } else {
        bevy::window::PresentMode::AutoNoVsync
    };
} 