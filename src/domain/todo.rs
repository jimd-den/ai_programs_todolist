use serde_derive::{Deserialize, Serialize};

/// Represents a Todo item with an ID, title, and completion status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub start_time: Option<String>, // New field for start time
    pub complete_time: Option<String>, // New field for complete time
}

impl Todo {
    /// Creates a new Todo item.
    pub fn new(id: i32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
            start_time: None,
            complete_time: None,
        }
    }

    /// Marks the Todo item as started.
    pub fn start(&mut self, start_time: String) {
        self.start_time = Some(start_time);
    }

    /// Marks the Todo item as completed.
    pub fn complete(&mut self, complete_time: String) {
        self.completed = true;
        self.complete_time = Some(complete_time);
    }
}