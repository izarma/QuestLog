use crate::model::epic::Epic;
use super::render_quests::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QuestLog {
    pub epics: Vec<Epic>,
    #[serde(skip)]
    pub screen_rect: Option<egui::Rect>,
    // Add state for tracking expanded quests
    pub expanded_quests: std::collections::HashSet<uuid::Uuid>,
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
}

impl eframe::App for QuestLog {
    /// Called each time the UI needs repainting, which may be many times per second  
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.screen_rect = Some(ctx.input(|input| input.screen_rect()));

        egui::Area::new("quest_overlay".into())
            .order(egui::Order::Foreground)
            .movable(false)
            .show(ctx, |ui| {
                render_quests(self, ui);
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
