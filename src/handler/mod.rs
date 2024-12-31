mod data;

use actix_web::{get, web, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    "helloo brou"
}