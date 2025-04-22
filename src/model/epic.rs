use super::quest::{Quest, QuestStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epic {
    pub id: Uuid,
    pub title: String,
    pub status: QuestStatus,
    pub quests: Vec<Quest>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
