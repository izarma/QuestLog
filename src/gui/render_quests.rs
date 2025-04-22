use crate::model::quest::{Quest, QuestStatus};
use super::app_setup::*;
use egui::Ui;

pub fn render_quest(quest: &Quest, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let (icon, color) = match quest.status {
            QuestStatus::Locked             => (egui::RichText::new("🔒"), egui::Color32::GRAY),
            QuestStatus::Available          => (egui::RichText::new("🟢"), egui::Color32::GREEN),
            QuestStatus::Planning           => (egui::RichText::new("📝"), egui::Color32::YELLOW),
            QuestStatus::OnHold             => (egui::RichText::new("⏸️"), egui::Color32::DARK_GRAY),
            QuestStatus::InProgress         => (egui::RichText::new("▶️"), egui::Color32::LIGHT_YELLOW),
            QuestStatus::CurrentlyTracking  => (egui::RichText::new("👀"), egui::Color32::LIGHT_GREEN),
            QuestStatus::Completed          => (egui::RichText::new("✔️"), egui::Color32::GREEN),
            QuestStatus::Discarded          => (egui::RichText::new("❌"), egui::Color32::RED),
        };

        ui.label(icon).on_hover_text(format!("Status: {:?}", quest.status));

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong(&quest.description);
                ui.add(
                    egui::ProgressBar::new(quest.progress)
                        .desired_width(100.0)
                        .fill(color)
                        .text(format!("{:.0}%", quest.progress * 100.0)),
                );
            });

            if let Some(deadline) = quest.deadline {
                let remaining = deadline - chrono::Utc::now();
                ui.label(format!("⏳ {} days left", remaining.num_days()));
            }

            for note in &quest.notes {
                ui.weak(note);   
            }
        });
    });
}

pub fn render_quest_tree(quest_log: &mut QuestLog, quest: &Quest, ui: &mut Ui) {
    let is_expanded = quest_log.expanded_quests.contains(&quest.id);
    let response = egui::CollapsingHeader::new(&quest.title)
        .id_salt(quest.id)
        .show(ui, |ui| {
            render_quest(quest, ui);
            if !quest.children.is_empty() {
                ui.indent("children", |ui| {
                    for child in &quest.children {
                        render_quest_tree(quest_log, child, ui);
                    }
                });
            }
        })
        .header_response;

    if response.clicked() {
        if is_expanded {
            quest_log.expanded_quests.remove(&quest.id);
        } else {
            quest_log.expanded_quests.insert(quest.id);
        }
    }
}

pub fn render_quests(quest_log: &mut QuestLog, ui: &mut Ui) {
    let screen_rect = quest_log.screen_rect.unwrap_or(ui.ctx().screen_rect());
    
    egui::Window::new("Active Quests")
        .title_bar(false)
        .resizable(false)
        .fixed_pos(egui::pos2(
            screen_rect.right() - 350.0,
            screen_rect.top() + 20.0,
        ))
        .frame(egui::Frame {
            inner_margin: egui::Margin::same(10),
            fill: egui::Color32::from_black_alpha(220),
            stroke: ui.visuals().widgets.noninteractive.bg_stroke,
            ..Default::default()
        })
        .show(ui.ctx(), |ui| {
            ui.heading("QuestLog");
            ui.separator();
            let cloned_self = quest_log.clone();
            let quests: Vec<_> = cloned_self
                .epics
                .iter()
                .flat_map(|epic| epic.quests.iter())
                .collect();

            for quest in quests {
                render_quest_tree(quest_log, quest, ui);
            }
        });
}