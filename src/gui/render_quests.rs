use super::{app_setup::*, render_content::PADDING};
use crate::model::quest::{Quest, QuestStatus};
use egui::{Id, Ui};

pub(crate) fn render_quest(ctx: &egui::Context, quest: &Quest, ui: &mut Ui) {
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
            ui.horizontal(|ui| {
                if let Some(deadline) = quest.deadline {
                    let remaining = deadline - chrono::Utc::now();
                    ui.label(format!("⏳ {} days left", remaining.num_days()));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.add_space(20.0);
                    let notes_btn = ui.add(egui::Button::new(
                        egui::RichText::new("📒").text_style(egui::TextStyle::Body),
                    ));
                    if notes_btn.clicked() {
                        show_notes_popup(ctx, quest, ui);
                    } else {
                    };
                });
            });

            ui.horizontal(|ui| {
                ui.label("Progress: ");
                ui.add(
                    egui::ProgressBar::new(quest.progress)
                        .desired_width(80.0)
                        .fill(color)
                        .text(format!("{:.0}%", quest.progress * 100.0)),
                );
                // add ui element to edit quest.progress
                ui.add(egui::Slider::new(&mut quest.progress.clone(), 0.0..=1.0)); // need to let this slider update progress
            });
        });
    });
}

pub(crate) fn render_quest_tree(
    ctx: &egui::Context,
    quest_log: &mut QuestLog,
    quest: &Quest,
    ui: &mut Ui,
) {
    let is_expanded = quest_log.expanded_quests.contains(&quest.id);
    let response = egui::CollapsingHeader::new(&quest.title)
        .id_salt(quest.id)
        .show(ui, |ui| {
            render_quest(ctx, quest, ui);
            if !quest.children.is_empty() {
                ui.vertical(|ui| {
                    for child in &quest.children {
                        render_quest_tree(ctx, quest_log, child, ui);
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

fn show_notes_popup(ctx: &egui::Context, quest: &Quest, ui: &mut Ui) {
    // Handle the button click event
    egui::Window::new("Notes 📒")
        .id(Id::new("Quest Notes"))
        .collapsible(true)
        .resizable(true)
        .anchor(egui::Align2::RIGHT_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label("Notes:");
            for note in &quest.notes {
                ui.weak(note);
            }
        });
}
