use egui::{FontData, FontDefinitions, FontFamily};

pub(crate) fn configure_font(ctx: &egui::Context) {
    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
        "NotoColor".to_string(),
        FontData::from_static(include_bytes!(
            //"../../assets/fonts/Noto_Color_Emoji/NotoColorEmoji-Regular.ttf"
            //"../../assets/fonts/Noto_Emoji/static/NotoEmoji-Light.ttf"
            "../../assets/fonts/Noto_Emoji/static/seguiemj.ttf"
        ))
        .into(),
    );

    font_def
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "NotoColor".to_string());

    ctx.set_fonts(font_def);
}
