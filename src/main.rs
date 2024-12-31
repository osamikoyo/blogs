use actix_web::{App, HttpServer};
use crate::handler::hello;

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let server = HttpServer::new(|| App::new().service(hello))
        .bind(("localhost:8080"))?
        .run();

    server.await
}
