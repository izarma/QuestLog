use crate::model::epic::Epic;
use crate::model::quest::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestLog {
    epics: Vec<Epic>,
    #[serde(skip)]
    screen_rect: Option<egui::Rect>,
    // Add state for tracking expanded quests
    expanded_quests: std::collections::HashSet<uuid::Uuid>,
}

impl QuestLog {
    pub fn new(cc: &eframe::CreationContext<'_>, initial_epic: Epic) -> Self {
        let mut log = Self {
            epics: vec![initial_epic],
            screen_rect: None,
            expanded_quests: std::collections::HashSet::new(),
        };
        // Load persisted state if needed
        if let Some(storage) = cc.storage {
            if let Some(saved) = eframe::get_value(storage, eframe::APP_KEY) {
                log = saved;
            }
        }
        log
    }

    fn render_quest(&self, quest: &Quest, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Status icon
            let (icon, color) = match quest.status {
                QuestStatus::Completed => ("✓", egui::Color32::GREEN),
                QuestStatus::InProgress => ("▶", egui::Color32::YELLOW),
                QuestStatus::Locked => ("🔒", egui::Color32::GRAY),
                _ => ("•", egui::Color32::WHITE),
            };

            ui.colored_label(color, icon);

            // Main quest info
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.strong(&quest.description);
                    ui.add(
                        egui::ProgressBar::new(quest.progress)
                            .desired_width(100.0)
                            .text(format!("{:.0}%", quest.progress * 100.0)),
                    );
                });

                // Deadline display
                if let Some(deadline) = quest.deadline {
                    let remaining = deadline - chrono::Utc::now();
                    ui.label(format!("⏳ {} days left", remaining.num_days()));
                }
            });
        });
    }

    fn render_quest_tree(&mut self, quest: &Quest, ui: &mut egui::Ui) {
        let is_expanded = self.expanded_quests.contains(&quest.id);
        let response = egui::CollapsingHeader::new("")
            .id_source(quest.id)
            .show(ui, |ui| {
                self.render_quest(quest, ui);
                if !quest.children.is_empty() {
                    ui.indent("children", |ui| {
                        for child in &quest.children {
                            self.render_quest_tree(child, ui);
                        }
                    });
                }
            })
            .header_response;

        if response.clicked() {
            if is_expanded {
                self.expanded_quests.remove(&quest.id);
            } else {
                self.expanded_quests.insert(quest.id);
            }
        }
    }

    fn render_quests(&mut self, ui: &mut egui::Ui) {
        let screen_rect = self.screen_rect.unwrap_or(ui.ctx().screen_rect());
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
                ui.heading("Current Quests");
                ui.separator();

                // Create a temporary collection first to avoid borrow conflicts
                let cloned_self = self.clone();
                let quests: Vec<_> = cloned_self
                    .epics
                    .iter()
                    .flat_map(|epic| epic.quests.iter())
                    .collect();

                for quest in quests {
                    self.render_quest_tree(quest, ui);
                }
            });
    }
}

impl eframe::App for QuestLog {
    /// Called each time the UI needs repainting, which may be many times per second  
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.screen_rect = Some(ctx.input(|input| input.screen_rect()));

        egui::Area::new("quest_overlay".into())
            .order(egui::Order::Foreground)
            .movable(false)
            .show(ctx, |ui| {
                self.render_quests(ui);
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
