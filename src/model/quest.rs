use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: Uuid,
    pub description: String,
    pub status: QuestStatus,
    pub notes: Vec<String>,
    pub progress: f32,                   // 0.0-1.0
    pub deadline: Option<DateTime<Utc>>, // Optional deadline
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dependencies: Vec<Uuid>,
    pub children: Vec<Quest>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum QuestStatus {
    Locked,
    Available,
    Planning,
    OnHold,
    InProgress,
    CurrentlyTracking,
    Completed,
    Discarded,
}

impl QuestStatus {
    pub fn iter() -> impl Iterator<Item = Self> {
        use QuestStatus::*;
        [
            Locked,
            Available,
            Planning,
            OnHold,
            InProgress,
            CurrentlyTracking,
            Completed,
            Discarded,
        ]
        .iter()
        .copied()
    }
}
