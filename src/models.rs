use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Task {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.updated_at = Utc::now();
    }

    pub fn update(&mut self, title: Option<String>, description: Option<String>) {
        if let Some(new_title) = title {
            self.title = new_title;
        }
        if let Some(new_desc) = description {
            self.description = Option::from(new_desc);
        }
        self.updated_at = Utc::now();
    }
}