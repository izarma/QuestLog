use crate::model::epic::Epic;
use crate::model::quest::*;
use uuid::Uuid;

pub fn create_dummy_data() -> (Epic, Epic) {
    let quest_1_1 = Quest {
        id: Uuid::new_v4(),
        title: "Rust Book Ch 1".to_string(),
        description: "Getting Started".to_string(),
        status: QuestStatus::Completed,
        notes: vec![
            "Read Rust Book Ch 1".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 1.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(1)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: Vec::new(),
    };

    let quest_1_2 = Quest {
        id: Uuid::new_v4(),
        title: "Rust Book Ch 2".to_string(),
        description: "Programming a Guessing Game".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Read Rust Book Ch 2".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 0.3,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(2)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: vec![quest_1_1.id.clone()],
        children: Vec::new(),
    };

    let quest_1_3 = Quest {
        id: Uuid::new_v4(),
        title: "Rust Book Ch 3".to_string(),
        description: "Common Programming Concepts".to_string(),
        status: QuestStatus::Locked,
        notes: vec![
            "Read Rust Book Ch 3".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(3)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: vec![quest_1_2.id.clone()],
        children: Vec::new(),
    };

    let quest = Quest {
        id: Uuid::new_v4(),
        title: "Rust Book".to_string(),
        description: "Read the Rust Programming Language Book End to End".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Read Rust Book".to_string(),
            "Watch LetsGetRusty Rust Book videos".to_string(),
            "Make and organize std rust notes".to_string(),
        ],
        progress: 1.3 / 20.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: vec![quest_1_1.clone(), quest_1_2.clone(), quest_1_3.clone()],
    };

    let quest_2_1 = Quest {
        id: Uuid::new_v4(),
        title: "Project QuestLog".to_string(),
        description: "Finish Building Quest Based Productivity Tool".to_string(),
        status: QuestStatus::CurrentlyTracking,
        notes: vec![
            "Create a screen overlay with egui".to_string(),
            "render quest tree".to_string(),
            "edit quests".to_string(),
        ],
        progress: 0.4,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: Vec::new(),
    };

    let quest_2_2 = Quest {
        id: Uuid::new_v4(),
        title: "make a copet".to_string(),
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
        title: "Rust GUI".to_string(),
        description: "Learn Rust GUI programming".to_string(),
        status: QuestStatus::InProgress,
        notes: vec![
            "Check native apps".to_string(),
            "check wasm apps".to_string(),
        ],
        progress: 0.3,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(10)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: vec![quest_2_1.clone(), quest_2_2.clone()],
    };

    let quest_3_1 = Quest {
        id: Uuid::new_v4(),
        title: "MCP Server".to_string(),
        description: "finish project MCP Server".to_string(),
        status: QuestStatus::Available,
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
        title: "Rust ML Ops".to_string(),
        description: "Learn Rust ML Ops programming".to_string(),
        status: QuestStatus::Available,
        notes: vec![
            "Check ONNX documentation".to_string(),
            "learn about MCP and A2A".to_string(),
        ],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(20)),
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

    let quest_a = Quest {
        id: Uuid::new_v4(),
        title: "Apply Online".to_string(),
        description: "Aply online with CA Number".to_string(),
        status: QuestStatus::Available,
        notes: vec!["check site".to_string()],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: Vec::new(),
    };

    let quest_a1 = Quest {
        id: Uuid::new_v4(),
        title: "Get vendor".to_string(),
        description: "Find and contact a vendor for installation".to_string(),
        status: QuestStatus::Locked,
        notes: vec!["check site".to_string()],
        progress: 0.0,
        deadline: Some(chrono::Utc::now() + chrono::Duration::days(7)),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        dependencies: Vec::new(),
        children: vec![quest_a.clone()],
    };
    let quests = vec![quest_a.clone(), quest_a1.clone()];
    let epic_2 = Epic {
        id: Uuid::new_v4(),
        title: "Solar Installation".to_string(),
        status: QuestStatus::Planning,
        quests,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    (epic, epic_2)
}
