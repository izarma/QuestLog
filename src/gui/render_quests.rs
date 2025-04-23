use super::app_setup::*;
use crate::model::quest::{Quest, QuestStatus};
use egui::Ui;

pub fn render_quest(quest: &Quest, ui: &mut Ui) {
    ui.horizontal(|ui| {
        let (icon, color) = match quest.status {
            QuestStatus::Locked => (egui::RichText::new("🔒"), egui::Color32::GRAY),
            QuestStatus::Available => (egui::RichText::new("🟢"), egui::Color32::GREEN),
            QuestStatus::Planning => (egui::RichText::new("📝"), egui::Color32::YELLOW),
            QuestStatus::OnHold => (egui::RichText::new("⏸️"), egui::Color32::DARK_GRAY),
            QuestStatus::InProgress => (egui::RichText::new("▶️"), egui::Color32::LIGHT_YELLOW),
            QuestStatus::CurrentlyTracking => {
                (egui::RichText::new("👀"), egui::Color32::LIGHT_GREEN)
            }
            QuestStatus::Completed => (egui::RichText::new("✔️"), egui::Color32::GREEN),
            QuestStatus::Discarded => (egui::RichText::new("❌"), egui::Color32::RED),
        };

        ui.label(icon)
            .on_hover_text(format!("Status: {:?}", quest.status));

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong(&quest.description);
            });

            if let Some(deadline) = quest.deadline {
                let remaining = deadline - chrono::Utc::now();
                ui.label(format!("⏳ {} days left", remaining.num_days()));
            }

            ui.horizontal(|ui| {
                ui.label("Progress: ");
                ui.add(
                    egui::ProgressBar::new(quest.progress)
                        .desired_width(80.0)
                        .fill(color)
                        .text(format!("{:.0}%", quest.progress * 100.0)),
                );
                // add ui element to edit quest.progress
                //ui.add(egui::Slider::new(mut quest.progress.clone(), 0.0..=1.0)); // need to let this slider update progress
            });

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
                ui.vertical( |ui| {
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