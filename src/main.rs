use actix_web::{App, HttpServer};
use crate::handler::{add_post, get_post, hello, update_post};

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let server = HttpServer::new(|| App::new().service(add_post)
        .service(get_post)
        .service(update_post))
        .bind(("localhost:8080"))?
        .run();

    server.await
}
