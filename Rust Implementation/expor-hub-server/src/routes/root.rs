use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Expor hub. Humble beginnings...")
}