use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use crate::adapters::controllers::TodoController;
use crate::application::use_cases::TodoService;
use crate::domain::todo::Todo;
use crate::infrastructure::db::SQLiteTodoRepository;

/// Sets up and runs the Actix web server.
pub async fn run_server() -> std::io::Result<()> {
    let db = SQLiteTodoRepository::new("todo.db");
    let service = TodoService::new(db);
    let controller = TodoController::new(service);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(controller.clone()))
            .route("/todos", web::get().to(get_all_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .service(Files::new("/", "./src/infrastructure/static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// Handler to get all Todo items.
async fn get_all_todos(controller: web::Data<TodoController<SQLiteTodoRepository>>) -> impl Responder {
    let todos = controller.get_all_todos();
    HttpResponse::Ok().json(todos)
}

/// Handler to create a new Todo item.
async fn create_todo(controller: web::Data<TodoController<SQLiteTodoRepository>>, title: web::Json<String>) -> impl Responder {
    let todo = controller.create_todo_request(title.into_inner());
    HttpResponse::Ok().json(todo)
}

/// Handler to get a Todo item by ID.
async fn get_todo(controller: web::Data<TodoController<SQLiteTodoRepository>>, id: web::Path<i32>) -> impl Responder {
    match controller.get_todo_request(id.into_inner()) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().finish(),
    }
}

/// Handler to update a Todo item.
async fn update_todo(controller: web::Data<TodoController<SQLiteTodoRepository>>, todo: web::Json<Todo>) -> impl Responder {
    controller.update_todo_request(todo.into_inner());
    HttpResponse::Ok().finish()
}

/// Handler to delete a Todo item by ID.
async fn delete_todo(controller: web::Data<TodoController<SQLiteTodoRepository>>, id: web::Path<i32>) -> impl Responder {
    controller.delete_todo_request(id.into_inner());
    HttpResponse::Ok().finish()
}
