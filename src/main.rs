mod domain;
mod application;
mod adapters;
mod infrastructure;

use actix_web::rt::System;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    infrastructure::web::run_server().await
}
