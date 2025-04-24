use super::{
    app_setup::*, panel_bottom::create_bottom_panel, panel_central::create_central_panel,
    panel_top::create_top_panel,
};
use eframe::egui::{self};
use egui::Ui;
pub const PADDING: f32 = 5.0;

pub fn render_content(quest_log: &mut QuestLog, ui: &mut Ui, ctx: &egui::Context) {
    create_top_panel(&ctx);
    create_central_panel(&ctx, ui, quest_log);
    create_bottom_panel(&ctx);
}
