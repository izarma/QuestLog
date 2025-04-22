use egui::Pos2;
use gui::app_setup::QuestLog;
use model::epic::*;
use model::quest::*;
use uuid::Uuid;

mod gui;
pub mod model;

fn calculate_top_right_position() -> Pos2 {
    let screen_size = egui::vec2(1920.0, 1080.0); // Example - implement dynamic detection
    egui::pos2(screen_size.x - 400.0 - 20.0, 20.0) // 400px width + 20px margin
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("QuestLog")
            .with_inner_size([400.0, 600.0])
            .with_min_inner_size([300.0, 400.0])
            .with_position(calculate_top_right_position())
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_window_level(egui::viewport::WindowLevel::AlwaysOnTop)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("..\\assets\\icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        persist_window: true,
        // default_theme: eframe::Theme::Dark,
        // mouse_passthrough: true, // Requires nightly
        ..Default::default()
    };
    let (epic, quest) = create_dummy_data();
    println!("Hello, Quest {:#?}", quest);
    println!("Hello, Quest Epic {:#?}", epic);
    eframe::run_native(
        "Quest Log Overlay",
        native_options,
        Box::new(|cc| Ok(Box::new(QuestLog::new(cc, epic)))),
    )
}

fn create_dummy_data() -> (Epic, Quest) {
    let quest_1_1 = Quest {
        id: Uuid::new_v4(),
        description: "Rust Book Ch 1".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Read Rust Book Ch 1".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 1.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: Vec::new(),
    };

    let quest_1_2 = Quest {
        id: Uuid::new_v4(),
        description: "Rust Book Ch 2".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Read Rust Book Ch 2".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 0.3,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: vec![quest_1_1.id.clone()],
        children: Vec::new(),
    };

    let quest = Quest {
        id: Uuid::new_v4(),
        description: "Rust Book".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Read Rust Book".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 0.3,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: vec![quest_1_1.clone(), quest_1_2.clone()],
    };

    let quest_2_1 = Quest {
        id: Uuid::new_v4(),
        description: "Project QuestLog".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Create a screen overlay with egui".to_string(),
            "render quest tree".to_string(),
            "edit quests".to_string(),
        ],
        progress: 1.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: Vec::new(),
    };

    let quest_2_2 = Quest {
        id: Uuid::new_v4(),
        description: "Make my Desktop Companion Pet".to_string(),
        status: QuestStatus::Locked,
        notes: vec![
            "have a gui overlay to render a sprite".to_string(),
            "let the sprite have basic interactions".to_string(),
        ],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: vec![quest_2_1.id.clone()],
        children: Vec::new(),
    };

    let quest_2 = Quest {
        id: Uuid::new_v4(),
        description: "Learn Rust GUI programming".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Check egui documentation".to_string(),
            "finish project QuestLog".to_string(),
        ],
        progress: 0.3,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: vec![quest_2_1.clone(), quest_2_2.clone()],
    };

    let quest_3_1 = Quest {
        id: Uuid::new_v4(),
        description: "finish project MCP Server".to_string(),
        status: QuestStatus::InProgress,
        notes: vec!["finish project MCP Server".to_string()],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: Vec::new(),
    };

    let quest_3 = Quest {
        id: Uuid::new_v4(),
        description: "Learn Rust ML Ops programming".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Check ONNX documentation".to_string(),
            "learn about MCP and A2A".to_string(),
        ],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: vec![quest_3_1.clone()],
    };

    let quests = vec![quest.clone(), quest_2.clone(), quest_3.clone()];

    let epic = Epic {
        id: Uuid::new_v4(),
        title: "Ultralearning Rust".to_string(),
        status: QuestStatus::InProgress,
        quests,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    (epic, quest)
}
