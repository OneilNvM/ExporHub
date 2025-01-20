use actix_web::{App, HttpServer};
use actix_web_lab::{header::StrictTransportSecurity, middleware::RedirectHttps};
use expor_hub_server::routes::root::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let _mw = RedirectHttps::with_hsts(StrictTransportSecurity::default().include_subdomains());

    println!("Server running at http://exporhub.com:9000");

    HttpServer::new(move || {
        App::new()
            .service(index)
    })
    .bind(("exporhub.com", 9000))?
    .run()
    .await
}


