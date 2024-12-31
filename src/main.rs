use actix_web::{App, HttpServer};
use crate::handler::{add_post, get_post, hello};

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let server = HttpServer::new(|| App::new().service(hello).service(add_post)
        .service(get_post))
        .bind(("localhost:8080"))?
        .run();

    server.await
}
