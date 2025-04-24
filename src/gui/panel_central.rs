use super::{app_setup::QuestLog, render_content::PADDING, render_quests::render_quest_tree};
use eframe::egui::{self, Color32};
use egui::{ScrollArea, Separator, Ui};

pub(crate) fn create_central_panel(ctx: &egui::Context, ui: &mut Ui, quest_log: &mut QuestLog) {
    let background_color = if ui.visuals().dark_mode {
        Color32::from_black_alpha(150)
    } else {
        Color32::from_additive_luminance(180)
    };
    egui::CentralPanel::default()
        .frame(egui::Frame {
            fill: background_color,
            stroke: ui.visuals().widgets.noninteractive.bg_stroke,
            ..Default::default()
        })
        .show(ui.ctx(), |ui| {
            render_header(ui);
            ScrollArea::vertical().show(ui, |ui| {
                render_epic_cards(ctx, ui, quest_log);
            });
        });
}

fn render_header(ui: &mut Ui) {
    ui.add_space(PADDING);
    ui.vertical_centered(|ui| {
        ui.heading("🔥 Quest Log 🔥");
    });
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn render_epic_cards(ctx: &egui::Context, ui: &mut Ui, quest_log: &mut QuestLog) {
    let log_loop = quest_log.clone();
    for epic in &log_loop.epics {
        ui.add_space(PADDING);
        ui.strong(format!(" {:?}", epic.title.clone()));
        ui.label(format!("  Status: {:?}", epic.status.clone()));
        for quest in &epic.quests {
            render_quest_tree(ctx, quest_log, quest, ui);
        }
        ui.separator();
    }
}
