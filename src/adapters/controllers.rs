use crate::application::use_cases::{TodoService, TodoRepository};
use crate::domain::todo::Todo;

#[derive(Clone)]
/// Controller for handling web requests for Todo items.
pub struct TodoController<T: TodoRepository> {
    service: TodoService<T>,
}

impl<T: TodoRepository> TodoController<T> {
    /// Creates a new TodoController with the given service.
    pub fn new(service: TodoService<T>) -> Self {
        Self { service }
    }

    /// Handles a request to create a new Todo item.
    pub fn create_todo_request(&self, title: String) -> Todo {
        self.service.create_todo(title)
    }

    /// Handles a request to get a Todo item by ID.
    pub fn get_todo_request(&self, id: i32) -> Option<Todo> {
        self.service.get_todo_by_id(id)
    }

    /// Handles a request to update a Todo item.
    pub fn update_todo_request(&self, todo: Todo) {
        self.service.update_todo(todo)
    }

    /// Handles a request to delete a Todo item by ID.
    pub fn delete_todo_request(&self, id: i32) {
        self.service.delete_todo_by_id(id)
    }

    /// Handles a request to get all Todo items.
    pub fn get_all_todos(&self) -> Vec<Todo> {
        self.service.get_all_todos()
    }
}
