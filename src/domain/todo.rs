// src/domain/todo.rs

use serde_derive::{Deserialize,Serialize};

/// Represents a Todo item with an ID, title, and completion status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    /// Creates a new Todo item.
    pub fn new(id: i32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    /// Marks the Todo item as completed.
    pub fn complete(&mut self) {
        self.completed = true;
    }
}
