use bevy::prelude::*;

mod math;
pub mod logger;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_utils);
    }
}

fn update_utils() {
    // Yardımcı fonksiyonlar güncelleme mantığı buraya gelecek
} 