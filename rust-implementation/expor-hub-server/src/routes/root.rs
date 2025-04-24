use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("2025 ExporHub API Server")
}
