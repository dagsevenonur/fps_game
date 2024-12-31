use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::ui::GameState;

#[derive(Resource)]
pub struct PlayerStats {
    pub health: f32,
    pub ammo: i32,
    pub fps: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 100.0,
            ammo: 30,
            fps: 0.0,
        }
    }
}

pub fn draw_hud(
    mut contexts: EguiContexts,
    stats: Res<PlayerStats>,
    state: Res<State<GameState>>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    if *state.get() == GameState::Paused {
        return;
    }

    let ctx = contexts.ctx_mut();
    let screen_size = ctx.screen_rect();

    // Crosshair (ekranın ortası)
    let crosshair_size = 20.0;
    let crosshair_stroke = 2.0;
    let center = screen_size.center();

    // Crosshair'i doğrudan çizelim
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("crosshair")));
    
    // Yatay çizgi
    painter.line_segment(
        [
            egui::pos2(center.x - 10.0, center.y),
            egui::pos2(center.x + 10.0, center.y),
        ],
        egui::Stroke::new(crosshair_stroke, egui::Color32::WHITE)
    );
    
    // Dikey çizgi
    painter.line_segment(
        [
            egui::pos2(center.x, center.y - 10.0),
            egui::pos2(center.x, center.y + 10.0),
        ],
        egui::Stroke::new(crosshair_stroke, egui::Color32::WHITE)
    );

    // FPS göstergesi (sol üst köşe)
    if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_value) = fps.smoothed() {
            egui::Window::new("FPS")
                .title_bar(false)
                .fixed_pos(egui::pos2(10.0, 10.0))
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    ui.colored_label(
                        egui::Color32::from_rgb(150, 255, 150),
                        egui::RichText::new(format!("FPS: {:.0}", fps_value))
                            .size(18.0)
                            .strong()
                    );
                });
        }
    }

    // Sağlık ve mermi göstergesi (sol alt köşe)
    egui::Window::new("Stats")
        .title_bar(false)
        .fixed_pos(egui::pos2(10.0, screen_size.max.y - 100.0))
        .frame(egui::Frame::none())
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Sağlık çubuğu
                let health_color = if stats.health > 70.0 {
                    egui::Color32::from_rgb(100, 255, 100)
                } else if stats.health > 30.0 {
                    egui::Color32::from_rgb(255, 255, 100)
                } else {
                    egui::Color32::from_rgb(255, 100, 100)
                };

                ui.horizontal(|ui| {
                    ui.colored_label(
                        egui::Color32::WHITE,
                        egui::RichText::new("HP")
                            .size(24.0)
                            .strong()
                    );
                    ui.add_space(10.0);

                    let progress_bar = egui::ProgressBar::new(stats.health / 100.0)
                        .desired_width(150.0)
                        .fill(health_color);
                    
                    ui.add_sized([150.0, 25.0], progress_bar);

                    ui.add_space(5.0);
                    ui.colored_label(
                        health_color,
                        egui::RichText::new(format!("{:.0}", stats.health))
                            .size(24.0)
                            .strong()
                    );
                });

                ui.add_space(10.0);

                // Mermi sayısı
                ui.horizontal(|ui| {
                    ui.colored_label(
                        egui::Color32::WHITE,
                        egui::RichText::new(format!("AMMO: {}", stats.ammo))
                            .size(24.0)
                            .strong()
                    );
                });
            });
        });
} 