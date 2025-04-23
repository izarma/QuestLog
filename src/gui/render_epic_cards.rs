use super::app_setup::*;
use super::render_quests::render_quest_tree;
use egui::{ScrollArea, Ui};

pub fn render_epic_cards(quest_log: &mut QuestLog, ui: &mut Ui, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::widgets::global_theme_preference_switch(ui);

            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                if ui.button("Exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                ui.add_space(16.0);
            }
        });
    });
    let screen_rect = quest_log.screen_rect.unwrap_or(ui.ctx().screen_rect());

    egui::Window::new("Quest Log")
        .title_bar(false)
        .resizable(false)
        .fixed_pos(egui::pos2(
            screen_rect.right() - 30.0,
            screen_rect.top() + 24.0,
        ))
        .frame(egui::Frame {
            inner_margin: egui::Margin::same(5),
            fill: egui::Color32::from_black_alpha(70), // need to change with theme
            stroke: ui.visuals().widgets.noninteractive.bg_stroke,
            ..Default::default()
        })
        .show(ui.ctx(), |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                let log_loop = quest_log.clone();
                for epic in &log_loop.epics {
                    ui.heading(epic.title.clone());
                    ui.label(format!("Status: {:?}", epic.status.clone()));
                    ui.label(format!("id: {:?}", epic.id.clone()));
                    for quest in &epic.quests {
                        render_quest_tree(quest_log, quest, ui);
                    }
                    ui.separator();
                }
            });
        });
}
