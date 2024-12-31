use bevy::prelude::*;

#[derive(Resource)]
pub struct RenderSettings {
    pub shadow_enabled: bool,
    pub msaa_samples: u32,
    pub hdr: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            shadow_enabled: true,
            msaa_samples: 4,
            hdr: true,
        }
    }
}

pub fn setup_renderer(mut commands: Commands) {
    commands.insert_resource(RenderSettings::default());
} 