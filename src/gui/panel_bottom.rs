use super::render_content::PADDING;
use eframe::egui::{self, Hyperlink, TopBottomPanel};
use egui::RichText;

pub(crate) fn create_bottom_panel(ctx: &egui::Context) {
    TopBottomPanel::bottom("footer panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(PADDING);
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("QuestLog Overlay 😸")
                    .small()
                    .text_style(egui::TextStyle::Body),
                "https://github.com/izarma/QuestLog",
            ));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("Made with egui")
                        .small()
                        .text_style(egui::TextStyle::Body),
                    "https://github.com/emilk/egui",
                ));
            });
            ui.add_space(PADDING);
        });
    });
}
