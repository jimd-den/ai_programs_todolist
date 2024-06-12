use rusqlite::{params, Connection, Result};
use crate::domain::todo::Todo;
use crate::application::use_cases::TodoRepository;

/// SQLite-based implementation of the TodoRepository.
pub struct SQLiteTodoRepository {
    conn: Connection,
}

impl Clone for SQLiteTodoRepository {
    fn clone(&self) -> Self {
        Self {
            conn: Connection::open("todo.db").unwrap(),
        }
    }
}

impl SQLiteTodoRepository {
    /// Creates a new SQLiteTodoRepository with a connection to the database.
    pub fn new(database_url: &str) -> Self {
        let conn = Connection::open(database_url).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            )",
            [],
        ).unwrap();
        Self { conn }
    }
}

impl TodoRepository for SQLiteTodoRepository {
    fn create(&self, title: String) -> Todo {
        self.conn.execute(
            "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
            params![title, false],
        ).unwrap();
        let id = self.conn.last_insert_rowid() as i32;
        Todo::new(id, title)
    }

    fn read(&self, id: i32) -> Option<Todo> {
        let mut stmt = self.conn.prepare("SELECT id, title, completed FROM todos WHERE id = ?1").unwrap();
        let todo_iter = stmt.query_map([id], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        }).unwrap();

        for todo in todo_iter {
            return Some(todo.unwrap());
        }
        None
    }

    fn update(&self, todo: Todo) {
        self.conn.execute(
            "UPDATE todos SET title = ?1, completed = ?2 WHERE id = ?3",
            params![todo.title, todo.completed, todo.id],
        ).unwrap();
    }

    fn delete(&self, id: i32) {
        self.conn.execute("DELETE FROM todos WHERE id = ?1", params![id]).unwrap();
    }

    fn read_all(&self) -> Vec<Todo> {
        let mut stmt = self.conn.prepare("SELECT id, title, completed FROM todos").unwrap();
        let todo_iter = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        }).unwrap();

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo.unwrap());
        }
        todos
    }
}
