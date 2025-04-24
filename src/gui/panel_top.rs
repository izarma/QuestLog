use eframe::egui::{self, Button, Label, Layout, TopBottomPanel};
use egui::RichText;

pub(crate) fn create_top_panel(ctx: &egui::Context) {
    TopBottomPanel::top("top panel").show(ctx, |ui| {
        ui.add_space(10.);
        egui::menu::bar(ui, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.add(Label::new(
                    RichText::new("📜").text_style(egui::TextStyle::Heading),
                ));
            });
            // controls
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_switch(ui);
                let add_btn = ui.add(Button::new(
                    RichText::new("➕").text_style(egui::TextStyle::Body),
                ));
                // if add_btn.clicked() {
                //     // Open a dialog to add a new epic
                //     if let Some(tx) = &self.app_tx {
                //         tx.send(Msg::NewEpic).expect("Failed to add Epic");
                //     }
                // }
                // config button
                let config_btn = ui.add(Button::new(
                    RichText::new("🛠").text_style(egui::TextStyle::Body),
                ));
                // about button
                let about_btn = ui.add(Button::new(
                    RichText::new("ℹ").text_style(egui::TextStyle::Body),
                ));
            });
        });
        ui.add_space(0.0);
    });
}
