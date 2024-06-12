use crate::domain::todo::Todo;

/// Defines the input boundary for interacting with todos.
pub trait TodoRepository {
    fn create(&self, title: String) -> Todo;
    fn read(&self, id: i32) -> Option<Todo>;
    fn update(&self, todo: Todo);
    fn delete(&self, id: i32);
    fn read_all(&self) -> Vec<Todo>;
}

#[derive(Clone)]
// Application service that provides use cases for Todo items.
pub struct TodoService<T: TodoRepository> {
    repository: T,
}

impl<T: TodoRepository> TodoService<T> {
    /// Creates a new TodoService with the given repository.
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    /// Creates a new Todo item.
    pub fn create_todo(&self, title: String) -> Todo {
        self.repository.create(title)
    }

    /// Reads a Todo item by ID.
    pub fn get_todo_by_id(&self, id: i32) -> Option<Todo> {
        self.repository.read(id)
    }

    /// Updates a Todo item.
    pub fn update_todo(&self, todo: Todo) {
        self.repository.update(todo)
    }

    /// Deletes a Todo item by ID.
    pub fn delete_todo_by_id(&self, id: i32) {
        self.repository.delete(id)
    }

    /// Reads all Todo items.
    pub fn get_all_todos(&self) -> Vec<Todo> {
        self.repository.read_all()
    }
}
